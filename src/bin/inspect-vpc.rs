use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{
    model::{Filter, Vpc},
    Client,
};

use anyhow::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let vpc = match std::env::args().nth(1) {
        Some(vpcid) => get_vpc(&client, &vpcid).await?,
        _ => get_default_vpc(&client).await?,
    };

    print_vpc_info(&vpc);

    Ok(())
}

async fn get_vpc(client: &Client, vpcid: &str) -> anyhow::Result<Vpc> {
    let filter = Filter::builder().name("vpc-id").values(vpcid).build();

    let resp = client.describe_vpcs().filters(filter).send().await?;

    if let Some(vpcs) = resp.vpcs() {
        if let Some(vpc) = vpcs.get(0) {
            return Ok(vpc.clone());
        }
    }

    Err(anyhow!(format!("Could not find VPC with ID: {}", &vpcid)))
}

async fn get_default_vpc(client: &Client) -> anyhow::Result<Vpc> {
    let filter = Filter::builder().name("is-default").values("true").build();

    let resp = client.describe_vpcs().filters(filter).send().await?;

    if let Some(vpcs) = resp.vpcs() {
        if let Some(vpc) = vpcs.get(0) {
            return Ok(vpc.clone());
        }
    }

    Err(anyhow!("Could not find a default VPC"))
}

fn print_vpc_info(vpc: &Vpc) {
    let vpcid = vpc.vpc_id().expect("a VPC should always have an ID");
    print!("VPC ID: {vpcid}");
    if vpc
        .is_default()
        .expect("a VPC should always have a flag for is_default")
    {
        println!(" (default)");
    } else {
        println!();
    }

    let accountid = vpc
        .owner_id()
        .expect("a VPC should always have an owner_id");
    println!("    Account ID: {accountid}");
}
