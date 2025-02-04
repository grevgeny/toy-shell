use core::str;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let cmd = {
            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue;
            }
            parse_command(trimmed_input)
        };

        execute_command(cmd)?;
    }
}

enum Command {
    Echo(String),
    Exit(i32),
    Type(String),
    Exec { programm: String, args: Vec<String> },
    Pwd,
    Cd(PathBuf),
    Unknown(String),
}

enum CommandKind {
    Builtin,
    Executable(PathBuf),
}

#[allow(deprecated)]
fn parse_command(input: &str) -> Command {
    let mut tokens = input.split_whitespace();

    match tokens.next() {
        Some("echo") => {
            let message = tokens.collect::<Vec<_>>().join(" ");
            Command::Echo(message)
        }
        Some("exit") => {
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
        Some("type") => {
            if let Some(cmd) = tokens.next() {
                Command::Type(cmd.to_string())
            } else {
                Command::Unknown(input.to_string())
            }
        }
        Some("pwd") => Command::Pwd,
        Some("cd") => {
            if let Some(path) = tokens.next() {
                if path == "~" {
                    Command::Cd(std::env::home_dir().unwrap())
                } else {
                    Command::Cd(path.into())
                }
            } else {
                Command::Unknown(input.to_string())
            }
        }
        Some(cmd) => {
            if let Some(CommandKind::Executable(_)) = resolve_command(cmd) {
                let args = tokens.map(String::from).collect::<Vec<_>>();
                Command::Exec {
                    programm: cmd.to_string(),
                    args,
                }
            } else {
                Command::Unknown(cmd.to_string())
            }
        }
        None => Command::Unknown(String::new()),
    }
}

fn resolve_command(name: &str) -> Option<CommandKind> {
    if matches!(name, "echo" | "exit" | "type" | "pwd" | "cd") {
        Some(CommandKind::Builtin)
    } else {
        env::var_os("PATH")
            .and_then(|paths| {
                env::split_paths(&paths).find_map(|dir| {
                    let full_path = dir.join(name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
            })
            .map(CommandKind::Executable)
    }
}

fn execute_command(command: Command) -> Result<(), anyhow::Error> {
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
                .output()?
                .stdout;
            let output_str = str::from_utf8(&output)?;
            print!("{output_str}");
        }
        Command::Pwd => {
            let wd = std::env::current_dir()?;
            println!("{}", wd.display());
        }

        Command::Cd(path) if path.exists() && path.is_dir() => std::env::set_current_dir(path)?,
        Command::Cd(path) => {
            println!("cd: {}: No such file or directory", path.display());
        }

        Command::Unknown(cmd) if !cmd.is_empty() => {
            println!("{cmd}: command not found");
        }
        Command::Unknown(_) => {}
    }

    Ok(())
}
