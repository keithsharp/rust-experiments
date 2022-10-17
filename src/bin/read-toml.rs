use std::env;
use std::io;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    profile: String,
    region: String,
}

fn main() {
    let mut args = env::args();
    let mut input = String::new();
    if args.len() > 1 {
        let name = args.nth(1).unwrap();
        File::open(&name)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();
    } else {
        io::stdin().read_to_string(&mut input).unwrap();
    }

    let config:Config = toml::from_str(input.as_str()).expect("Could not deserialise");
    println!("Profile: {}, Region: {}", config.profile, config.region);
}