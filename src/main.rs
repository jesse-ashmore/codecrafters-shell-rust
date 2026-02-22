#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let command = prompt();
        println!("{}: command not found", command.trim());
        io::stdout().flush().unwrap();
    }
}

fn prompt() -> String {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command.trim().to_string()
}
