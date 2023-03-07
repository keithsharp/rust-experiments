use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::model::{Instance, InstanceType, ResourceType, Tag, TagSpecification};
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
        .expect("Failed to get VPC ID from VPC")
        .to_string();

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
        .expect("Failed to get Subnet ID from Subnet")
        .to_string();

    let resp = client
        .run_instances()
        .instance_type(InstanceType::T3Micro)
        .image_id("ami-065793e81b1869261")
        .min_count(1)
        .max_count(1)
        .subnet_id(subnetid)
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
