use clap::{Args, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello(HelloArgs),
}

#[derive(Args)]
struct HelloArgs {
    names: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello(args) => {
            if args.names.len() < 1 {
                Cli::command()
                    .find_subcommand_mut("hello")
                    .expect("should always be able to find 'hello' subcommand")
                    .print_help()?;
                std::process::exit(1);
            }
            for name in &args.names {
                println!("Hello, {}!", name);
            }
        }
    }

    Ok(())
}
