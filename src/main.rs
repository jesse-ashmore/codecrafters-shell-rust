#[allow(unused_imports)]
use std::io::{self, Write};
use std::vec;

const EMPTY_STRING: String = String::new();
struct Args(Vec<String>);
trait Command {
    fn evaluate(&self, args: Args) -> Option<String>;
}
enum Cmd {
    Noop,
    Exit,
    NotFound(String),
    Echo,
}

impl Command for Cmd {
    fn evaluate(&self, args: Args) -> Option<String> {
        match self {
            Cmd::Exit => None,
            Cmd::NotFound(cmd) => Some(format!("{}: command not found", cmd)),
            Cmd::Echo => Some(args.0.join(" ")),
            Cmd::Noop => Some(EMPTY_STRING),
        }
    }
}

fn main() {
    loop {
        let (cmd, args) = read();

        if let Some(output) = cmd.evaluate(args) {
            println!("{output}");
            io::stdout().flush().unwrap();
        } else {
            break;
        }
    }
}

fn read() -> (Cmd, Args) {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split(" ");

    if let Some(command) = parts.next() {
        let func = match command.trim() {
            "exit" => Cmd::Exit,
            "echo" => Cmd::Echo,
            other => Cmd::NotFound(other.to_string()),
        };

        let args: Vec<String> = match func {
            Cmd::Echo => parts.map(|s| s.to_string()).collect(),
            _ => vec![],
        };

        return (func, Args(args));
    }

    (Cmd::Noop, Args(vec![]))
}
