use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{types::Filter, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Create VPC
    let resp = client.create_vpc().cidr_block("10.0.0.0/16").send().await?;

    let vpcid = resp
        .vpc()
        .expect("should always get a VPC object back")
        .vpc_id()
        .expect("should always get a VPC ID from a VPC object");
    println!("Created VPC: {}", vpcid);

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
    println!("Got Route Table ID: {}", rtid);

    // Create a subnet
    let resp = client
        .create_subnet()
        .vpc_id(vpcid)
        .cidr_block("10.0.0.0/24")
        .send()
        .await?;

    let subnetid = resp
        .subnet()
        .expect("should always get a Subnet object back")
        .subnet_id()
        .expect("should always get a Subnet ID from a Subnet");
    println!("Created Subnet: {}", subnetid);

    // Create Internet Gateway
    let resp = client.create_internet_gateway().send().await?;

    let igid = resp
        .internet_gateway()
        .expect("should always get an Internet Gateway")
        .internet_gateway_id()
        .expect("an Internet Gateway should always have an ID");
    println!("Created Internet Gateway: {}", igid);

    // Attach to VPC
    client
        .attach_internet_gateway()
        .internet_gateway_id(igid)
        .vpc_id(vpcid)
        .send()
        .await?;
    println!("Attached {} to {}", igid, vpcid);

    // Add a route to the Internet
    client
        .create_route()
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(igid)
        .route_table_id(rtid)
        .send()
        .await?;
    println!("Added a default route to {} via {}", rtid, igid);

    Ok(())
}
