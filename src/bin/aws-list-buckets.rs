use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();

    for bucket in buckets {
        println!("Bucket: {}", bucket.name().unwrap_or("Unamed"));
        let tag_res = client
            .get_bucket_tagging()
            .bucket(bucket.name().unwrap_or_default())
            .send()
            .await;

        match tag_res {
            Ok(resp) => {
                let tag_set = resp.tag_set().unwrap_or_default();
                for tag in tag_set {
                    let key = tag.key().unwrap_or_default();
                    let value = tag.value().unwrap_or_default();
                    println!("\tTag Name: {}, Tag Value: {}", key, value);
                }
            }
            Err(_) => continue, // Should really match this to NoSuchTagSet
        }
    }

    Ok(())
}
