use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{model::Filter, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let filter = Filter::builder().name("is-default").values("true").build();

    let resp = client.describe_vpcs().filters(filter).send().await?;

    let vpcid = resp
        .vpcs()
        .expect("do I always get an array, even if no match?")
        .get(0)
        .expect("assume I have a default VPC")
        .vpc_id()
        .expect("should always get a VPC ID");
    println!("Got default VPC: {}", &vpcid);

    // Create the first security group
    let resp = client
        .create_security_group()
        .group_name("SG-One")
        .description("First Security Group")
        .vpc_id(vpcid)
        .send()
        .await?;

    let sg_one_id = resp
        .group_id()
        .expect("should always get a security group ID back");

    // Describe the first Security Group and get it's details
    let resp = client
        .describe_security_groups()
        .group_ids(sg_one_id)
        .send()
        .await?;

    let sg = resp
        .security_groups()
        .expect("do I always get an array, even if no match?")
        .get(0)
        .expect("assume no one has deleted SG-One");

    let sg_one_name = sg
        .group_name()
        .expect("should always get a security group name");
    let sg_one_id = sg
        .group_id()
        .expect("should always get a security group ID");
    let sg_one_account = sg.owner_id().expect("should always get an account");
    let sg_one_vpc = sg.vpc_id().expect("should always get a VPC ID");

    println!(
        "Created {} with name {} in {} in {}",
        sg_one_id, sg_one_name, sg_one_vpc, sg_one_account
    );

    // Create the second Security Group
    let resp = client
        .create_security_group()
        .group_name("SG-Two")
        .description("Second Security Group")
        .vpc_id(vpcid)
        .send()
        .await?;

    let sg_two_id = resp
        .group_id()
        .expect("should always get a security group ID back");

    // Describe the second Security Group and get it's details
    let resp = client
        .describe_security_groups()
        .group_ids(sg_two_id)
        .send()
        .await?;

    let sg = resp
        .security_groups()
        .expect("do I always get an array, even if no match?")
        .get(0)
        .expect("assume no one has deleted SG-Two");

    let sg_two_name = sg
        .group_name()
        .expect("should always get a security group name");
    let sg_two_id = sg
        .group_id()
        .expect("should always get a security group ID");
    let sg_two_account = sg.owner_id().expect("should always get an account");
    let sg_two_vpc = sg.vpc_id().expect("should always get a VPC ID");

    println!(
        "Created {} with name {} in {} in {}",
        sg_two_id, sg_two_name, sg_two_vpc, sg_two_account
    );

    // Add an ingress rule to SG-Two allowing access from SG-One
    client
        .authorize_security_group_ingress()
        .group_id(sg_two_id)
        // This shouldn't be needed, but just in case
        .source_security_group_owner_id(sg_one_account)
        .source_security_group_name(sg_one_name)
        .send()
        .await?;

    Ok(())
}
