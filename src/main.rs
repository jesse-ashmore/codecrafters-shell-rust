#[allow(unused_imports)]
use std::io::{self, Write};

struct Args(Vec<String>);

fn main() {
    while let Some(command) = evaluate(&read()) {
        io::stdout().flush().unwrap();
    }
}

fn read() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}

fn evaluate(command: &str) -> Option<impl Fn()> {
    return match command {
        "exit" => None,
        other => Some(|| not_found(other)),
    };
}

fn not_found(command: &str) {
    println!("{}: command not found", command.trim());
}
