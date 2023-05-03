use aws_config::meta::region::RegionProviderChain;

use aws_sdk_iam::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let trust_policy = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "EC2AssumeRole",
            "Effect": "Allow",
            "Principal": {
                "Service": "ec2.amazonaws.com"
            },
            "Action": "sts:AssumeRole"
        }
    ]
}"#;

    let resp = client
        .create_role()
        .role_name("TestInstanceProfile")
        .description("Allow Listing, Putting, and Getting objects from a specific S3 bucket")
        .assume_role_policy_document(trust_policy)
        .send()
        .await?;

    let role = resp.role().expect("should always get a Role struct back");
    let role_name = role.role_name().expect("should always get a Role name");
    println!("Created role {}", role_name);

    let policy = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "ListObjectsInBucket",
            "Effect": "Allow",
            "Action": ["s3:ListBucket"],
            "Resource": ["arn:aws:s3:::bucket-name"]
        },
        {
            "Sid": "AllObjectActions",
            "Effect": "Allow",
            "Action": ["s3:PutObject", "s3:GetObject"],
            "Resource": ["arn:aws:s3:::bucket-name/*"]
        }
    ]
}"#;

    client
        .put_role_policy()
        .role_name(role_name)
        .policy_name("S3AccessPolicy")
        .policy_document(policy)
        .send()
        .await?;
    println!("Added policy to role: {}", role_name);

    let resp = client
        .create_instance_profile()
        .instance_profile_name("TestInstanceProfile")
        .send()
        .await?;

    let profile_name = resp
        .instance_profile()
        .expect("should always get an Instance Profile")
        .instance_profile_name()
        .expect("should always get an instance profile name");
    println!("Created Instance Profile {}", profile_name);

    client
        .add_role_to_instance_profile()
        .instance_profile_name(profile_name)
        .role_name(role_name)
        .send()
        .await?;
    println!(
        "Assigned Role {} to Instance Profile {}",
        role_name, profile_name
    );

    Ok(())
}
