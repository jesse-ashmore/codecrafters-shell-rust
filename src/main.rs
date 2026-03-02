#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    os::{self, unix::fs::PermissionsExt},
    path::{self, Path, PathBuf},
    vec,
};

use itertools::Itertools;

const EMPTY_STRING: String = String::new();
struct Args(Vec<String>);
trait Command {
    fn evaluate(&self, env: &Environment, args: Args) -> Option<String>;
}

#[derive(PartialEq, Eq)]
enum Cmd {
    Noop,
    Exit(String),
    NotFound(String),
    Echo(String),
    Type(String),
    External(String, PathBuf),
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
    fn evaluate(&self, env: &Environment, args: Args) -> Option<String> {
        match self {
            Cmd::Exit(_) => None,
            Cmd::NotFound(cmd) => Some(format!("{}: command not found", cmd)),
            Cmd::Echo(_) => Some(args.0.join(" ")),
            Cmd::Noop => Some(EMPTY_STRING),
            Cmd::Type(_) => env.builtin_type(args),
            Cmd::External(_, _) => todo!("Must implement evaluate for Cmd::External"),
        }
    }
}

fn main() {
    loop {
        let env = Environment::new();
        let (cmd, args) = env.read();

        if let Some(output) = cmd.evaluate(&env, args) {
            println!("{output}");
            io::stdout().flush().unwrap();
        } else {
            break;
        }
    }
}

fn get_paths() -> Option<Vec<PathBuf>> {
    let key = "PATH";
    if let Some(paths) = env::var_os(key) {
        return Some(env::split_paths(&paths).collect_vec());
    }

    println!("{key} is not defined in the environment.");
    None
}

struct Environment {
    paths: Vec<PathBuf>,
}

impl Environment {
    fn read(&self) -> (Cmd, Args) {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.parse_cmd(&input)
    }

    fn parse_cmd(&self, input: &str) -> (Cmd, Args) {
        let mut parts = input.trim().split(" ");

        if let Some(command) = parts.next() {
            let name = command.trim();
            let func = Cmd::from_name(name);

            if let Some(builtin) = func {
                return (builtin, Args(parts.map(|s| s.to_string()).collect()));
            }

            for dir in &self.paths {
                let full_path = dir.join(name);
                // Check if file exists and is executable

                if full_path.exists() {
                    if let Ok(metadata) = full_path.metadata() {
                        let permissions = metadata.permissions();
                        let mode = permissions.mode();
                        if mode & 0o111 != 0 {
                            return (
                                Cmd::External(name.to_string(), full_path),
                                Args(parts.map(|s| s.to_string()).collect()),
                            );
                        }
                    }
                }
            }

            return (Cmd::NotFound(name.to_string()), Args(vec![]));
        }

        (Cmd::Noop, Args(vec![]))
    }

    fn builtin_type(&self, args: Args) -> Option<String> {
        let interpret = self.parse_cmd(&args.0.join(" "));

        match interpret.0 {
            Cmd::Noop => Cmd::NotFound(EMPTY_STRING).evaluate(&self, args),
            Cmd::NotFound(name) => Some(format!("{name}: not found")),
            Cmd::External(name, path) => Some(format!("{name}: is {}", path.to_str().unwrap())),
            _ => Some(format!("{} is a shell builtin", &interpret.0.get_name())),
        }
    }

    fn new() -> Self {
        Environment {
            paths: get_paths().unwrap_or(vec![]),
        }
    }
}
