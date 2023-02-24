use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{model::Filter, Client};

use anyhow::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let vpcid = match std::env::args().nth(1) {
        Some(vpcid) => vpcid,
        _ => get_default_vpc_id(&client).await?,
    };
    println!("{vpcid}");

    Ok(())
}

async fn get_default_vpc_id(client: &Client) -> anyhow::Result<String> {
    let filter = Filter::builder().name("is-default").values("true").build();

    let resp = client.describe_vpcs().filters(filter).send().await?;

    if let Some(vpcs) = resp.vpcs() {
        if let Some(vpc) = vpcs.get(0) {
            let vpcid = vpc.vpc_id().expect("should always get a VPC ID");
            return Ok(vpcid.to_owned());
        }
    }

    Err(anyhow!("Could not find a default VPC"))
}
