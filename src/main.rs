#[allow(unused_imports)]
use std::io::{self, Write};

struct Args(Vec<String>);
trait Command {
    fn evaluate(&self, args: Args) -> Option<String>;
}
enum Cmd {
    Exit,
    NotFound(String),
}

impl Command for Cmd {
    fn evaluate(&self, args: Args) -> Option<String> {
        match self {
            Cmd::Exit => None,
            Cmd::NotFound(cmd) => Some(format!("{}: command not found", cmd)),
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
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();

    let func = match command.trim() {
        "exit" => Cmd::Exit,
        other => Cmd::NotFound(other.to_string()),
    };

    let args = Args(vec![]);

    (func, args)
}
