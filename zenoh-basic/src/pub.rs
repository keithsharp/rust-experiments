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

    if let Err(e) = session.put(KEY_EXPR, "Keith").await {
        eprintln!("Failed to send message: {}", e)
    } else {
        println!("Sent message successfully")
    }

    if let Err(e) = session.close().await {
        eprintln!("Failed to close the session: {}", e)
    }

    println!("Exiting program")
}
