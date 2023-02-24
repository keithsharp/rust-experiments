use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{model::Filter, Client, Error};

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

    // Create the S3 Gateway Endpoint
    let service = "com.amazonaws.eu-west-1.s3";

    client
        .create_vpc_endpoint()
        .vpc_id(vpcid)
        .route_table_ids(rtid)
        .service_name(service)
        .send()
        .await?;

    Ok(())
}
