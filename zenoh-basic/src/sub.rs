mod common;
use crate::common::KEY_EXPR;

#[tokio::main]
async fn main() {
    let session = match zenoh::open(zenoh::Config::default()).await {
        Ok(session) => {
            println!("Got session {:?}", session);
            session
        }
        Err(e) => {
            eprintln!("Failed to create a session: {}", e);
            std::process::exit(2)
        }
    };

    let subscriber = match session.declare_subscriber(KEY_EXPR).await {
        Ok(subscriber) => {
            println!("Got a subscriber: {:?}", subscriber);
            subscriber
        }
        Err(e) => {
            eprintln!("Failed to create a subscriber: {}", e);
            std::process::exit(2)
        }
    };

    while let Ok(sample) = subscriber.recv_async().await {
        match sample.payload().try_to_string() {
            Ok(name) => println!("Hello, {}", name),
            Err(e) => eprintln!("Failed to get name from sample: {}", e),
        }
    }
}
