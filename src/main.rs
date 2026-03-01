#[allow(unused_imports)]
use std::io::{self, Write};
use std::vec;

const EMPTY_STRING: String = String::new();
struct Args(Vec<String>);
trait Command {
    fn evaluate(&self, args: Args) -> Option<String>;
}

#[derive(PartialEq, Eq)]
enum Cmd {
    Noop,
    Exit(String),
    NotFound(String),
    Echo(String),
    Type(String),
    External(String, String),
}

impl Cmd {
    fn get_name(&self) -> String {
        match self {
            Cmd::Noop => EMPTY_STRING,
            Cmd::Exit(name) | Cmd::NotFound(name) | Cmd::Echo(name) | Cmd::Type(name) => {
                name.to_string()
            }
            Cmd::External(name, _) => name.to_string(),
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        match name {
            "exit" => Some(Cmd::Exit(name.to_string())),
            "echo" => Some(Cmd::Echo(name.to_string())),
            "type" => Some(Cmd::Type(name.to_string())),
            _ => None,
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
            Cmd::Type(_) => builtin_type(args),
            Cmd::External(_, _) => todo!("Must implement evaluate for Cmd::External"),
        }
    }
}

fn builtin_type(args: Args) -> Option<String> {
    let interpret = parse_cmd(&args.0.join(" "));

    match interpret.0 {
        Cmd::Noop => Cmd::NotFound(EMPTY_STRING).evaluate(args),
        Cmd::NotFound(name) => Some(format!("{name}: not found")),
        Cmd::External(name, path) => Some(format!("{name}: is {path}")),
        _ => Some(format!("{} is a shell builtin", &interpret.0.get_name())),
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
        let name = command.trim();
        let func = Cmd::from_name(name);

        if let Some(builtin) = func {
            return (builtin, Args(parts.map(|s| s.to_string()).collect()));
        }

        if let Some(path) = get_path(name) {
            return (
                Cmd::External(name.to_string(), path),
                Args(parts.map(|s| s.to_string()).collect()),
            );
        }

        return (Cmd::NotFound(name.to_string()), Args(vec![]));
    }

    (Cmd::Noop, Args(vec![]))
}

fn get_path(cmd: &str) -> Option<String> {
    todo!("If the command is not a builtin, your shell must go through every directory in PATH. For each directory:

    Check if a file with the command name exists.
    Check if the file has execute permissions.
    If the file exists and has execute permissions, print <command> is <full_path> and stop.
    If the file exists but lacks execute permissions, skip it and continue to the next directory.")
}
