use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};

fn add<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    let first = args.get_one::<String>("X").unwrap();
    let first = first.parse::<f32>()?;
    let second = args.get_one::<String>("Y").unwrap();
    let second = second.parse::<f32>()?;
    let result = first + second;

    Ok(Some(result.to_string()))
}

fn subtract<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    let first = args.get_one::<String>("X").unwrap();
    let first = first.parse::<f32>()?;
    let second = args.get_one::<String>("Y").unwrap();
    let second = second.parse::<f32>()?;
    let result = second - first;

    Ok(Some(result.to_string()))
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("Repl")
        .with_version("0.1.0")
        .with_description("Testing REPLs")
        .with_banner("This is my REPL")
        .with_hinter_disabled()
        .with_command(
            Command::new("add")
                .arg(Arg::new("X").required(true).allow_negative_numbers(true))
                .arg(Arg::new("Y").required(true).allow_negative_numbers(true))
                .about("add X to Y"),
            add,
        )
        .with_command(
            Command::new("subtract")
                .arg(Arg::new("X").required(true).allow_negative_numbers(true))
                .arg(Arg::new("Y").required(true).allow_negative_numbers(true))
                .about("subtract X from Y"),
            subtract,
        );

    repl.run()?;

    Ok(())
}
