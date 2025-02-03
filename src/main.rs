use std::io::{self, Write};

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
    Unknown(String),
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "echo" | "exit" | "type")
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
            if let Ok(code) = tokens.collect::<String>().parse::<i32>() {
                Command::Exit(code)
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
        _ => Command::Unknown(cmd.to_string()),
    }
}

fn execute_command(command: Command) {
    match command {
        Command::Echo(msg) => {
            println!("{msg}");
        }
        Command::Exit(code) => std::process::exit(code),
        Command::Type(cmd) if is_builtin(&cmd) => {
            println!("{cmd} is a shell builtin");
        }
        Command::Type(cmd) => {
            println!("{cmd}: not found")
        }
        Command::Unknown(cmd) if !cmd.is_empty() => {
            println!("{cmd}: command not found");
        }
        Command::Unknown(_) => {}
    }
}
