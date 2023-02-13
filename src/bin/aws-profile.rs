use std::env;

use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;

use aws_sdk_ec2::Client;
use aws_sdk_ec2::Error;
use aws_sdk_ec2::Region;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let profile = env::args()
        .skip(1)
        .next()
        .expect("profile to use is required");

    let region = DefaultRegionChain::builder()
        .profile_name(&profile)
        .build()
        .region()
        .await
        .or(Some(Region::new("eu-west-1")));

    let creds = DefaultCredentialsChain::builder()
        .profile_name(&profile)
        .region(region.clone())
        .build()
        .await;

    let config = aws_config::from_env()
        .credentials_provider(creds)
        .region(region)
        .load()
        .await;

    let client = Client::new(&config);

    let req = client.describe_vpcs();
    let resp = req.send().await?;
    println!("{:?}", resp);

    Ok(())
}
