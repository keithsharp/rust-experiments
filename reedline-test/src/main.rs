use reedline::{DefaultPrompt, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        reedline::DefaultPromptSegment::Basic("reedline-test ".to_string()),
        reedline::DefaultPromptSegment::CurrentDateTime,
    );

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => match line.as_ref() {
                "help" | "Help" => {
                    println!("Sorry, there is not help");
                }
                "quit" | "Quit" => {
                    println!("Got 'quit' command, quitting");
                    break;
                }
                _ => {
                    println!("Got line: '{}'", line);
                }
            },
            Ok(Signal::CtrlC | Signal::CtrlD) => {
                println!("Got CTRL-C or CTRL-D, quitting");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    println!("Would run clean up here.");
}
