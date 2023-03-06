use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error};

use uuid::Uuid;

const PREFIX: &str = "test";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = match std::env::args().nth(1) {
        Some(file) => file,
        _ => {
            println!("Usage: s3-file-upload <FILENAME>");
            std::process::exit(1);
        }
    };
    let path = Path::new(&file);
    let key = path
        .file_name()
        .expect("Path should always have a final component")
        .to_string_lossy();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let bucket_name = Uuid::new_v4();

    let constraint = BucketLocationConstraint::from("eu-west-1");
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    client
        .create_bucket()
        .bucket(bucket_name.hyphenated().to_string())
        .create_bucket_configuration(cfg)
        .send()
        .await?;
    println!("Created bucket {}", bucket_name.hyphenated().to_string());

    let body = ByteStream::from_path(path).await;
    let key = PREFIX.to_string() + "/" + &key;
    client
        .put_object()
        .bucket(bucket_name.hyphenated().to_string())
        .key(key)
        .body(body.unwrap())
        .send()
        .await?;

    let resp = client
        .list_objects_v2()
        .bucket(bucket_name.hyphenated().to_string())
        .prefix(PREFIX)
        .send()
        .await?;

    let keys: Vec<&str> = resp
        .contents()
        .unwrap_or_default()
        .iter()
        .map(|o| o.key().unwrap_or_default())
        .collect();

    println!("Files in bucket:");
    for key in keys {
        println!("  {key}");
    }

    Ok(())
}
