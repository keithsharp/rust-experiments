use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{types::Filter, Client};

use anyhow::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let vpc = match std::env::args().nth(1) {
        Some(vpcid) => Vpc::new_from_vpc_id(&client, &vpcid).await?,
        _ => Vpc::default_vpc(&client).await?,
    };

    vpc.print_info();

    Ok(())
}

// My own VPC struct to unwrap the AWS SDK into
pub struct Vpc {
    id: String,
    account: String,
    default: bool,
}

impl Vpc {
    pub fn new_from_vpc(vpc: &aws_sdk_ec2::types::Vpc) -> Self {
        let id = vpc
            .vpc_id()
            .expect("a VPC should always have an ID")
            .to_owned();
        let account = vpc
            .owner_id()
            .expect("a VPC should always have an owner_id")
            .to_owned();
        let default = vpc
            .is_default()
            .expect("a VPC should always have a flag for is_default")
            .to_owned();

        Self {
            id,
            account,
            default,
        }
    }

    async fn default_vpc(client: &Client) -> anyhow::Result<Vpc> {
        let filter = Filter::builder().name("is-default").values("true").build();

        let resp = client.describe_vpcs().filters(filter).send().await?;

        if let Some(vpcs) = resp.vpcs() {
            if let Some(vpc) = vpcs.get(0) {
                return Ok(Vpc::new_from_vpc(vpc));
            }
        }

        Err(anyhow!("Could not find a default VPC"))
    }

    async fn new_from_vpc_id(client: &Client, vpcid: &str) -> anyhow::Result<Vpc> {
        let filter = Filter::builder().name("vpc-id").values(vpcid).build();

        let resp = client.describe_vpcs().filters(filter).send().await?;

        if let Some(vpcs) = resp.vpcs() {
            if let Some(vpc) = vpcs.get(0) {
                return Ok(Vpc::new_from_vpc(vpc));
            }
        }

        Err(anyhow!(format!("Could not find VPC with ID: {}", &vpcid)))
    }
}

impl Vpc {
    pub fn vpc_id(&self) -> String {
        self.id.clone()
    }

    pub fn account_id(&self) -> String {
        self.account.clone()
    }

    pub fn is_default(&self) -> bool {
        self.default
    }
}

impl Vpc {
    fn print_info(&self) {
        print!("VPC ID: {}", self.id);
        if self.default {
            println!(" (default)");
        } else {
            println!();
        }
        println!("    Account ID: {}", self.account);
    }
}
