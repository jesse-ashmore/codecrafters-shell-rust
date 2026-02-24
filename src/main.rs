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
    Exit(String),
    NotFound(String),
    Echo(String),
    Type(String),
}

impl Cmd {
    fn get_name(&self) -> String {
        match self {
            Cmd::Noop => EMPTY_STRING,
            Cmd::Exit(name) | Cmd::NotFound(name) | Cmd::Echo(name) | Cmd::Type(name) => {
                name.to_string()
            }
        }
    }
}

impl Command for Cmd {
    fn evaluate(&self, args: Args) -> Option<String> {
        match self {
            Cmd::Exit(_) => None,
            Cmd::NotFound(cmd) => Some(format!("{}: command not found", cmd)),
            Cmd::Echo(_) => Some(args.0.join(" ")),
            Cmd::Noop => Some(EMPTY_STRING),
            Cmd::Type(_) => {
                let interpret = parse_cmd(&args.0.join(" "));
                match interpret.0 {
                    Cmd::Noop => Cmd::NotFound(EMPTY_STRING).evaluate(args),
                    Cmd::NotFound(_) => interpret.0.evaluate(args),
                    _ => Some(format!("{} is a shell builtin", &interpret.0.get_name())),
                }
            }
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
    parse_cmd(&input)
}

fn parse_cmd(input: &str) -> (Cmd, Args) {
    let mut parts = input.trim().split(" ");

    if let Some(command) = parts.next() {
        let trimmed = command.trim();
        let func = match trimmed {
            "exit" => Cmd::Exit(trimmed.to_string()),
            "echo" => Cmd::Echo(trimmed.to_string()),
            "type" => Cmd::Type(trimmed.to_string()),
            other => Cmd::NotFound(other.to_string()),
        };

        let args: Vec<String> = match func {
            Cmd::Echo(_) => parts.map(|s| s.to_string()).collect(),
            _ => vec![],
        };

        return (func, Args(args));
    }

    (Cmd::Noop, Args(vec![]))
}
