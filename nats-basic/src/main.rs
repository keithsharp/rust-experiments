use std::env;

use anyhow::anyhow;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let Some(command) = args.get(1) else {
        return Err(anyhow!("Usage: nats-basic pub|sub <SUBJECT> [Message]"))
    };

    let Some(subject) = args.get(2) else {
        return Err(anyhow!("Usage: nats-basic pub|sub <SUBJECT> [Message]"))
    };

    let client = async_nats::connect("localhost").await?;

    match command.as_str() {
        "pub" => {
            let Some(payload) = args.get(3) else {
                return Err(anyhow!("Usage: nats-basic pub|sub <SUBJECT> [Message]"))
            };
            println!("Publishing message '{}' to subject '{}'", payload, subject);
            client
                .publish(subject.into(), payload.to_owned().into())
                .await?;
        }
        "sub" => {
            println!("Listening on subject: '{}'", subject);
            let mut subscriber = client.subscribe(subject.to_owned()).await?;
            while let Some(message) = subscriber.next().await {
                let payload = String::from_utf8_lossy(&(message.payload.to_vec())).to_string();
                match payload.as_str() {
                    "QUIT" => {
                        println!("Got QUIT message, quitting.");
                        break;
                    }
                    _ => println!("Received message {:?}", payload),
                }
            }
        }
        _ => return Err(anyhow!("Usage: nats-basic pub|sub <SUBJECT> [Message]")),
    }

    Ok(())
}
