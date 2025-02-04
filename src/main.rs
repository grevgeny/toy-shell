use core::str;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = {
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue;
            }
            parse_command(trimmed_input)
        };

        execute_command(command);
    }
}

enum Command {
    Echo(String),
    Exit(i32),
    Type(String),
    Exec { programm: String, args: Vec<String> },
    Pwd,
    Unknown(String),
}

enum CommandKind {
    Builtin,
    Executable(PathBuf),
}

fn parse_command(input: &str) -> Command {
    let mut tokens = input.split_whitespace();

    let cmd = match tokens.next() {
        Some(token) => token,
        None => return Command::Unknown(String::new()),
    };

    match cmd {
        "echo" => {
            let message = tokens.collect::<Vec<_>>().join(" ");
            Command::Echo(message)
        }
        "exit" => {
            if let Some(code_str) = tokens.next() {
                if let Ok(code) = code_str.parse::<i32>() {
                    Command::Exit(code)
                } else {
                    Command::Unknown(input.to_string())
                }
            } else {
                Command::Unknown(input.to_string())
            }
        }
        "type" => {
            if let Some(cmd) = tokens.next() {
                Command::Type(cmd.to_string())
            } else {
                Command::Unknown(input.to_string())
            }
        }
        "pwd" => Command::Pwd,
        cmd => {
            let Some(CommandKind::Executable(_)) = resolve_command(cmd) else {
                return Command::Unknown(cmd.to_string());
            };

            let args = tokens.map(String::from).collect::<Vec<_>>();
            Command::Exec {
                programm: cmd.to_string(),
                args,
            }
        }
    }
}

fn resolve_command(name: &str) -> Option<CommandKind> {
    if matches!(name, "echo" | "exit" | "type" | "pwd") {
        Some(CommandKind::Builtin)
    } else {
        env::var_os("PATH")
            .and_then(|paths| {
                env::split_paths(&paths)
                    .filter_map(|dir| {
                        let full_path = dir.join(name);
                        if full_path.is_file() {
                            Some(full_path)
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .map(CommandKind::Executable)
    }
}

fn execute_command(command: Command) {
    match command {
        Command::Echo(msg) => {
            println!("{msg}");
        }
        Command::Exit(code) => std::process::exit(code),
        Command::Type(cmd) => match resolve_command(&cmd) {
            Some(CommandKind::Builtin) => println!("{cmd} is a shell builtin"),
            Some(CommandKind::Executable(path)) => println!("{cmd} is {}", path.display()),
            None => println!("{cmd}: not found"),
        },
        Command::Exec { programm, args } => {
            let output = std::process::Command::new(programm)
                .args(args)
                .output()
                .unwrap()
                .stdout;
            let output_str = str::from_utf8(&output).unwrap();
            print!("{output_str}");
        }
        Command::Pwd => {
            let wd = std::env::current_dir().unwrap();
            println!("{}", wd.display());
        }
        Command::Unknown(cmd) if !cmd.is_empty() => {
            println!("{cmd}: command not found");
        }
        Command::Unknown(_) => {}
    }
}
