#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command;
    io::stdin().read_line(&mut command).unwrap();

    println!("{command}: command not found");
}
