use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    name: String,
    #[clap(short, long, action)]
    debug: bool,
}

fn main() {
    let args = Arguments::parse();

    if args.debug {
        println!("{:?}", args);
    }
    println!("Hello, {}", args.name);
}
