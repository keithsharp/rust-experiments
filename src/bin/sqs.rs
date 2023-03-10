use clap::{Args, Parser, Subcommand};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::Client;

use aws_arn::ResourceName;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Create(CreateArgs),
    Delete(DeleteArgs),
    Describe(DescribeArgs),
    List,
    Message(MessageArgs),
}

#[derive(Args)]
struct CreateArgs {
    name: String,
}

#[derive(Args)]
struct DeleteArgs {
    name: String,
}

#[derive(Args)]
struct DescribeArgs {
    name: String,
}

#[derive(Args)]
struct MessageArgs {
    #[clap(subcommand)]
    command: MessageCommand,
    #[clap(short)]
    queue: String,
}

#[derive(Subcommand)]
enum MessageCommand {
    Send(SendArgs),
    Get,
}

#[derive(Args)]
struct SendArgs {
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let cli = Cli::parse();
    match cli.command {
        Command::Create(args) => create_queue(&client, &args.name).await?,
        Command::Delete(args) => delete_queue(&client, &args.name).await?,
        Command::Describe(args) => describe_queue(&client, &args.name).await?,
        Command::List => list_queues(&client).await?,
        Command::Message(args) => match args.command {
            MessageCommand::Send(message) => {
                send_message(&client, &args.queue, &message.message).await?
            }
            MessageCommand::Get => get_message(&client, &args.queue).await?,
        },
    }

    Ok(())
}

async fn list_queues(client: &Client) -> anyhow::Result<()> {
    let resp = client.list_queues().send().await?;

    if let Some(queue_urls) = resp.queue_urls() {
        if queue_urls.len() > 0 {
            for url in queue_urls {
                print_queue(client, url).await?;
            }
            return Ok(());
        }
    }
    println!("No SQS queues in this region.");
    Ok(())
}

async fn print_queue(client: &Client, url: &str) -> anyhow::Result<()> {
    let name = url_to_name(client, url).await?;
    println!("'{}' '{}'", name, url);

    Ok(())
}

async fn url_to_name(client: &Client, url: &str) -> anyhow::Result<String> {
    let resp = client
        .get_queue_attributes()
        .queue_url(url)
        .attribute_names(aws_sdk_sqs::model::QueueAttributeName::QueueArn)
        .send()
        .await?;

    let attributes = resp
        .attributes()
        .expect("should always get queue attributes");

    match attributes.get(&aws_sdk_sqs::model::QueueAttributeName::QueueArn) {
        Some(arn) => {
            let arn: ResourceName = arn.parse().expect("should always get a valid ARN");
            println!("{} {}", arn.resource, url);
            Ok(arn.resource.to_string())
        }
        None => return Err(anyhow::anyhow!(format!("Could not get name for {}", &url))),
    }
}

async fn name_to_url(client: &Client, name: &str) -> anyhow::Result<String> {
    let resp = client.get_queue_url().queue_name(name).send().await?;

    if let Some(url) = resp.queue_url() {
        return Ok(url.to_owned());
    }

    Err(anyhow::anyhow!(format!("Could not find URL for {}", name)))
}

async fn create_queue(client: &Client, name: &str) -> anyhow::Result<()> {
    let resp = client.create_queue().queue_name(name).send().await?;

    let queue_url = resp.queue_url().expect("queue should have a URL");

    println!("Created '{}' with URL '{}'", name, queue_url);

    Ok(())
}

async fn delete_queue(client: &Client, name: &str) -> anyhow::Result<()> {
    let url = name_to_url(client, name).await?;
    client.delete_queue().queue_url(url).send().await?;
    Ok(())
}

async fn describe_queue(client: &Client, name: &str) -> anyhow::Result<()> {
    let url = name_to_url(client, name).await?;

    let resp = client
        .get_queue_attributes()
        .attribute_names(aws_sdk_sqs::model::QueueAttributeName::All)
        .queue_url(&url)
        .send()
        .await?;

    let attributes = resp
        .attributes()
        .expect("should always get queue attributes");

    let arn = attributes
        .get(&aws_sdk_sqs::model::QueueAttributeName::QueueArn)
        .expect("queue should always have an ARN");
    let message_count = attributes
        .get(&aws_sdk_sqs::model::QueueAttributeName::ApproximateNumberOfMessages)
        .expect("queue should always have a message count even if it's zero");

    println!("{}", name);
    println!("    URL: {}", url);
    println!("    ARN: {}", arn);
    println!("    Message count: {}", message_count);

    Ok(())
}

async fn send_message(client: &Client, name: &str, message: &str) -> anyhow::Result<()> {
    let url = name_to_url(client, name).await?;

    let resp = client
        .send_message()
        .queue_url(&url)
        .message_body(message)
        .send()
        .await?;

    let message_id = resp.message_id().expect("should always get a message ID");
    println!("Sent message '{}' to '{}", message_id, name);

    Ok(())
}

async fn get_message(client: &Client, name: &str) -> anyhow::Result<()> {
    let url = name_to_url(client, name).await?;

    let resp = client
        .receive_message()
        .queue_url(&url)
        .max_number_of_messages(1)
        .send()
        .await?;

    if let Some(messages) = resp.messages() {
        if messages.len() > 0 {
            for message in messages {
                println!(
                    "Message '{}' '{}'",
                    message
                        .message_id()
                        .expect("a message should have a message ID"),
                    message.body().unwrap_or("No message body")
                );
                client
                    .delete_message()
                    .queue_url(&url)
                    .receipt_handle(
                        message
                            .receipt_handle()
                            .expect("a message should have a receipt handle"),
                    )
                    .send()
                    .await?;
            }
            return Ok(());
        }
    }

    println!("No messages in queue '{}'", name);

    Ok(())
}
