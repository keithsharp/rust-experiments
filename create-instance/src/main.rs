use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::types::{
    AttributeBooleanValue, Filter, IamInstanceProfileSpecification, Instance, InstanceType,
    ResourceType, ShutdownBehavior, Tag, TagSpecification,
};
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_iam::Client as IamClient;

use base64::{engine::general_purpose, Engine as _};

use std::{thread, time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let ec2_client = Ec2Client::new(&config);
    let iam_client = IamClient::new(&config);

    let tags = vec![
        Tag::builder()
            .key("project")
            .value("create-instance")
            .build(),
        Tag::builder()
            .key("Name")
            .value("EC2 Instance Testing")
            .build(),
    ];

    // Create the Role
    let trust_policy = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Sid": "EC2AssumeRole",
                "Effect": "Allow",
                "Principal": {
                    "Service": "ec2.amazonaws.com"
                },
                "Action": "sts:AssumeRole"
            }
        ]
    }"#;

    let resp = iam_client
        .create_role()
        .role_name("TestInstanceProfile")
        .description("Allow Listing, Putting, and Getting objects from a specific S3 bucket")
        .assume_role_policy_document(trust_policy)
        .send()
        .await?;

    let role = resp.role().expect("should always get a Role struct back");
    let role_name = role.role_name().expect("should always get a Role name");
    println!("Created role {}", role_name);

    // Attach a Policy to the Role
    let policy = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Sid": "AllowFullS3Access",
                "Effect": "Allow",
                "Action": ["s3:*"],
                "Resource": ["*"]
            }
        ]
    }"#;

    iam_client
        .put_role_policy()
        .role_name(role_name)
        .policy_name("S3AccessPolicy")
        .policy_document(policy)
        .send()
        .await?;
    println!("Added policy to role: {}", role_name);

    // Create the Instance Profile
    let resp = iam_client
        .create_instance_profile()
        .instance_profile_name("TestInstanceProfile")
        .send()
        .await?;

    let profile_name = resp
        .instance_profile()
        .expect("should always get an Instance Profile")
        .instance_profile_name()
        .expect("should always get an instance profile name");
    println!("Created Instance Profile {}", profile_name);

    // Connect the Role and the Instance Profile
    iam_client
        .add_role_to_instance_profile()
        .instance_profile_name(profile_name)
        .role_name(role_name)
        .send()
        .await?;
    println!(
        "Assigned Role {} to Instance Profile {}",
        role_name, profile_name
    );

    // Create the VPC
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Vpc)
        .set_tags(Some(tags.clone()))
        .build();

    let resp = ec2_client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .tag_specifications(tag_spec)
        .send()
        .await?;

    let vpcid = resp
        .vpc()
        .expect("Failed to get VPC from create_vpc() response")
        .vpc_id()
        .expect("Failed to get VPC ID from VPC");

    ec2_client
        .modify_vpc_attribute()
        .vpc_id(vpcid)
        .enable_dns_hostnames(AttributeBooleanValue::builder().value(true).build())
        .send()
        .await?;

    println!("Created VPC: {}", vpcid);

    // Create a subnet
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Subnet)
        .set_tags(Some(tags.clone()))
        .build();

    let resp = ec2_client
        .create_subnet()
        .vpc_id(vpcid)
        .cidr_block("10.0.0.0/24")
        .tag_specifications(tag_spec)
        .send()
        .await?;

    let subnetid = resp
        .subnet()
        .expect("Failed to get Subnet from create_subnet() response")
        .subnet_id()
        .expect("Failed to get Subnet ID from Subnet");

    ec2_client
        .modify_subnet_attribute()
        .subnet_id(subnetid)
        .map_public_ip_on_launch(AttributeBooleanValue::builder().value(true).build())
        .send()
        .await?;

    println!("Created Subnet: {}", subnetid);

    // Get the ID of the main Route Table
    let vpc_id_filter = Filter::builder().name("vpc-id").values(vpcid).build();
    let main_route_table_filter = Filter::builder()
        .name("association.main")
        .values("true")
        .build();

    let resp = ec2_client
        .describe_route_tables()
        .filters(vpc_id_filter)
        .filters(main_route_table_filter)
        .send()
        .await?;

    let rtid = resp
        .route_tables()
        .expect("should always get a vec of route tables")
        .get(0)
        .expect("should always have one main route table")
        .route_table_id()
        .expect("main route table should always have an ID");

    println!("Got Route Table ID: {}", rtid);

    // Create Internet Gateway
    let resp = ec2_client.create_internet_gateway().send().await?;

    let igid = resp
        .internet_gateway()
        .expect("should always get an Internet Gateway")
        .internet_gateway_id()
        .expect("an Internet Gateway should always have an ID");

    ec2_client
        .attach_internet_gateway()
        .internet_gateway_id(igid)
        .vpc_id(vpcid)
        .send()
        .await?;

    println!("Attached {} to {}", igid, vpcid);

    // Add a route to the Internet
    ec2_client
        .create_route()
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(igid)
        .route_table_id(rtid)
        .send()
        .await?;

    println!("Added a default route to {} via {}", rtid, igid);

    // Create a Security Group
    let resp = ec2_client
        .create_security_group()
        .group_name("SSH Allowed")
        .description("Allow SSH from anywhere")
        .vpc_id(vpcid)
        .send()
        .await?;

    let sgid = resp
        .group_id()
        .expect("should always get a security group ID back");

    ec2_client
        .authorize_security_group_ingress()
        .group_id(sgid)
        .ip_protocol("tcp")
        .cidr_ip("0.0.0.0/0") // Anywhere!
        .from_port(22) // SSH
        .to_port(22)
        .send()
        .await?;

    println!("Created Security Group: {}", sgid);

    // Launch an instance
    // Log to serial console and file.
    // Taken from: https://aws.amazon.com/premiumsupport/knowledge-center/ec2-linux-log-user-data/
    let userdata = r##"#!/bin/bash -xe
    exec > >(tee /var/log/user-data.log|logger -t user-data -s 2>/dev/console) 2>&1
    mkdir /blender
    cd /blender
    curl -s -L https://download.blender.org/release/Blender3.4/blender-3.4.1-linux-x64.tar.xz -o blender-3.4.1-linux-x64.tar.xz
    tar xf blender-3.4.1-linux-x64.tar.xz
    yum -y install libX11 libXrender libXxf86vm libXfixes libXi libxkbcommon
"##;

    let userdata = general_purpose::STANDARD_NO_PAD.encode(userdata);
    let instance_profile = IamInstanceProfileSpecification::builder()
        .name(profile_name)
        .build();

    // It takes a while for changes to IAM (the Role, Policy, and Instance Profile)
    // to reach eventual consistency across all of the AWS Regions.  The proper solution
    // is Waiters: https://github.com/awslabs/aws-sdk-rust/issues/400
    let duration = 5;
    println!("Sleeping for {duration} seconds to allow the IAM Instance Profile to propagate");
    std::thread::sleep(std::time::Duration::from_secs(duration));

    let resp = ec2_client
        .run_instances()
        .instance_type(InstanceType::T3Micro)
        .image_id("ami-065793e81b1869261")
        .min_count(1)
        .max_count(1)
        .subnet_id(subnetid)
        .security_group_ids(sgid)
        .key_name("rust-test")
        .instance_initiated_shutdown_behavior(ShutdownBehavior::Terminate)
        .iam_instance_profile(instance_profile)
        .user_data(userdata)
        .send()
        .await?;

    let instances: Vec<String> = resp
        .instances()
        .expect("instances should have been created")
        .iter()
        .map(|i| {
            i.instance_id()
                .expect("instance should always have an ID")
                .to_string()
        })
        .collect();

    // Wait for the Instance to move from Pending to Running
    let delay = time::Duration::from_secs(5);
    loop {
        let resp = ec2_client
            .describe_instances()
            .set_instance_ids(Some(instances.clone()))
            .send()
            .await?;

        let instances = resp
            .reservations()
            .expect("should always have an array of reservations")
            .get(0)
            .expect("should always have a single reservation")
            .instances()
            .expect("should always have an array of instances")
            .to_owned();

        let pending: Vec<Instance> = instances
            .into_iter()
            .filter(|i| {
                i.state()
                    .expect("instance should always have a state")
                    .code()
                    == Some(0)
            })
            .collect();

        for instance in &pending {
            println!(
                "Instance {} is pending",
                instance
                    .instance_id()
                    .expect("instance should always have an ID")
            );
        }

        if pending.len() == 0 {
            break;
        }
        thread::sleep(delay);
    }

    for instance in instances {
        println!("Instance {} is running", instance);
    }

    Ok(())
}
