use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello(HelloArgs),
    Goodbye(GoodbyeArgs),
}

#[derive(Args)]
struct HelloArgs {
    #[clap(default_value_t = String::from("World"))]
    name: String,
}

#[derive(Args)]
struct GoodbyeArgs {
    #[clap(subcommand)]
    commands: GoodbyeCommands,
}

#[derive(Subcommand)]
enum GoodbyeCommands {
    Final,
    Temporary,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello(args) => {
            println!("Hello, {}!", args.name);
        },
        Commands::Goodbye(args) => {
            match args.commands {
                GoodbyeCommands::Final => {
                    println!("It's a final goodbye.");
                },
                GoodbyeCommands::Temporary => {
                    println!("We'll meet again soon.")
                }
            }
        }
    }
}