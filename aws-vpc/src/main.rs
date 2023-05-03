use aws_sdk_ec2::{types::Tag, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Describe the VPCs
    let req = client.describe_vpcs();
    let resp = req.send().await?;
    println!("{:?}", resp);

    // Create a Tag
    let tag = Tag::builder().key("Project").value("AWS Rust Test").build();

    // Apply the Tag to a specific VPC
    let add_tags_req = client
        .create_tags()
        .resources("vpc-0be713be661b7db37")
        .tags(tag.clone());
    let add_tags_resp = add_tags_req.send().await?;
    println!("{:?}", add_tags_resp);

    // Describe the VPCs
    let req = client.describe_vpcs();
    let resp = req.send().await?;
    println!("{:?}", resp);

    // Delete the Tag from the VPC
    let del_tags_req = client
        .delete_tags()
        .resources("vpc-0be713be661b7db37")
        .tags(tag);
    let del_tags_resp = del_tags_req.send().await?;
    println!("{:?}", del_tags_resp);

    // Describe the VPCs
    let req = client.describe_vpcs();
    let resp = req.send().await?;
    println!("{:?}", resp);

    Ok(())
}
