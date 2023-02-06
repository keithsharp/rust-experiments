use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error};
use aws_sdk_ec2::model::{Filter, Tag, TagSpecification, ResourceType};

#[cfg(debug_assertions)]
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(not(debug_assertions))]
    env_logger::init();
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let tags = vec![
        Tag::builder()
            .key("project")
            .value("aws-create-vpc")
            .build(),
        Tag::builder()
            .key("Name")
            .value("My first Rust VPC")
            .build()
    ];

    let vpcid = create_vpc(&client, &tags).await?;
    info!("Created VPC: {}", vpcid);

    let subnetid = create_subnet(&client, &vpcid, &tags).await?;
    info!("Created Subnet: {}", subnetid);

    let rtid = get_main_route_table(&client, &vpcid).await?;
    info!("Main Route Table: {}", rtid);

    Ok(())
}

async fn create_vpc(client: &Client, tags: &Vec<Tag>) -> Result<String, Error> {
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Vpc)
        .set_tags(Some(tags.clone()))
        .build();
    
    let resp = client.create_vpc()
        .cidr_block("10.0.0.0/16")
        .tag_specifications(tag_spec)
        .send()
        .await?;

    let vpcid = resp.vpc()
        .expect("Failed to get VPC from create_vpc() response")
        .vpc_id()
        .expect("Failed to get VPC ID from VPC")
        .to_string();

    Ok(vpcid)
}

async fn get_main_route_table(client: &Client, vpcid: &str) -> Result<String, Error> {
    let vpc_id_filter = Filter::builder()
        .name("vpc-id")
        .values(vpcid)
        .build();

    let main_route_table_filter = Filter::builder()
        .name("association.main")
        .values("true")
        .build();

    let resp = client.describe_route_tables()
        .filters(vpc_id_filter)
        .filters(main_route_table_filter)
        .send()
        .await?;
    
    let route_tables = resp.route_tables()
        .expect("Failed to get Route Tables from describe_route_tables() response");
    
    if route_tables.len() != 1 {
        panic!("Got {} Route Tables for VPC {}, was only expecting 1", route_tables.len(), vpcid);
    }
    
    let rtid = route_tables[0].route_table_id()
        .expect("Failed to get Route Table ID from RouteTable")
        .to_string();

    Ok(rtid)
}

async fn create_subnet(client: &Client, vpcid: &str, tags: &Vec<Tag>) -> Result<String, Error> {
    let tag_spec = TagSpecification::builder()
        .resource_type(ResourceType::Subnet)
        .set_tags(Some(tags.clone()))
        .build();
    
    let resp = client.create_subnet()
        .vpc_id(vpcid)
        .cidr_block("10.0.0.0/24")
        .tag_specifications(tag_spec)
        .send()
        .await?;
    
    let subnetid = resp.subnet()
        .expect("Failed to get Subnet from create_subnet() response")
        .subnet_id()
        .expect("Failed to get Subnet ID from Subnet")
        .to_string();

    Ok(subnetid)
}