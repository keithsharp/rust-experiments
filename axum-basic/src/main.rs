use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long, default_value = "127.0.0.1")]
    address: Ipv4Addr,
    #[clap(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    println!("Listening on http://{}:{}", &args.address, &args.port);

    let app = Router::new().route("/", get(get_root)).fallback(fallback);

    let socket = SocketAddr::new(IpAddr::V4(args.address), args.port);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_root(uri: Uri) -> String {
    format!("Hello from {}", uri)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Page not found: {}", uri))
}
