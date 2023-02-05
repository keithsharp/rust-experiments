use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::model::{CreateBucketConfiguration, BucketLocationConstraint, Tag, Tagging};

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let bucket_name = Uuid::new_v4();

    let constraint = BucketLocationConstraint::from("eu-west-1");
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    let resp = client.create_bucket()
        .bucket(bucket_name.hyphenated().to_string())
        .create_bucket_configuration(cfg)
        .send()
        .await?;
    println!("Created bucket: {}", resp.location().unwrap());

    let tag = Tag::builder().key("project").value("aws-create-bucket").build();
    let tagging = Tagging::builder()
        .tag_set(tag).
        build();
    
    client.put_bucket_tagging()
        .bucket(bucket_name.hyphenated().to_string())
        .tagging(tagging)
        .send()
        .await?;

    client.delete_bucket()
        .bucket(bucket_name.hyphenated().to_string())
        .send()
        .await?;

    Ok(())
}
