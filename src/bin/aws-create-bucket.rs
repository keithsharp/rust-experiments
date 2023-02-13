use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration, Tag, Tagging};
use aws_sdk_s3::{Client, Error};

#[cfg(debug_assertions)]
use env_logger::Env;
use log::info;

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(not(debug_assertions))]
    env_logger::init();

    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let bucket_name = Uuid::new_v4();

    let constraint = BucketLocationConstraint::from("eu-west-1");
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    info!("Creating bucket: {}", bucket_name.hyphenated().to_string());
    client
        .create_bucket()
        .bucket(bucket_name.hyphenated().to_string())
        .create_bucket_configuration(cfg)
        .send()
        .await?;

    let tag = Tag::builder()
        .key("project")
        .value("aws-create-bucket")
        .build();
    let tagging = Tagging::builder().tag_set(tag.clone()).build();

    info!(
        "Adding tag {}:{} to bucket {}",
        tag.key().unwrap(),
        tag.value().unwrap(),
        bucket_name.hyphenated().to_string()
    );
    client
        .put_bucket_tagging()
        .bucket(bucket_name.hyphenated().to_string())
        .tagging(tagging)
        .send()
        .await?;

    info!("Deleting bucket: {}", bucket_name.hyphenated().to_string());
    client
        .delete_bucket()
        .bucket(bucket_name.hyphenated().to_string())
        .send()
        .await?;

    info!("All done.");
    Ok(())
}
