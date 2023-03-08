use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::model::{
    AttributeBooleanValue, Filter, Instance, InstanceType, ResourceType, ShutdownBehavior, Tag,
    TagSpecification,
};
use aws_sdk_ec2::{Client, Error};

use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

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

    // Create the VPC
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Vpc)
        .set_tags(Some(tags.clone()))
        .build();

    let resp = client
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

    client
        .modify_vpc_attribute()
        .vpc_id(vpcid)
        .enable_dns_hostnames(AttributeBooleanValue::builder().value(true).build())
        .send()
        .await?;

    // Create a subnet
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Subnet)
        .set_tags(Some(tags.clone()))
        .build();

    let resp = client
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

    client
        .modify_subnet_attribute()
        .subnet_id(subnetid)
        .map_public_ip_on_launch(AttributeBooleanValue::builder().value(true).build())
        .send()
        .await?;

    // Get the ID of the main Route Table
    let vpc_id_filter = Filter::builder().name("vpc-id").values(vpcid).build();
    let main_route_table_filter = Filter::builder()
        .name("association.main")
        .values("true")
        .build();

    let resp = client
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

    // Create Internet Gateway
    let resp = client.create_internet_gateway().send().await?;

    let igid = resp
        .internet_gateway()
        .expect("should always get an Internet Gateway")
        .internet_gateway_id()
        .expect("an Internet Gateway should always have an ID");

    // Attach Internet Gateway to VPC
    client
        .attach_internet_gateway()
        .internet_gateway_id(igid)
        .vpc_id(vpcid)
        .send()
        .await?;

    // Add a route to the Internet
    client
        .create_route()
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(igid)
        .route_table_id(rtid)
        .send()
        .await?;

    // Create a Security Group
    let resp = client
        .create_security_group()
        .group_name("SSH Allowed")
        .description("Allow SSH from anywhere")
        .vpc_id(vpcid)
        .send()
        .await?;

    let sgid = resp
        .group_id()
        .expect("should always get a security group ID back");

    client
        .authorize_security_group_ingress()
        .group_id(sgid)
        .ip_protocol("tcp")
        .cidr_ip("0.0.0.0/0") // Anywhere!
        .from_port(22) // SSH
        .to_port(22)
        .send()
        .await?;

    // Launch an instance
    let resp = client
        .run_instances()
        .instance_type(InstanceType::T3Micro)
        .image_id("ami-065793e81b1869261")
        .min_count(1)
        .max_count(1)
        .subnet_id(subnetid)
        .security_group_ids(sgid)
        .key_name("rust-test")
        .instance_initiated_shutdown_behavior(ShutdownBehavior::Terminate)
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
        let resp = client
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

    println!("Instances are all running.");

    Ok(())
}
