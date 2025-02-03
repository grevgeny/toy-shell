use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (cmd, args) = {
            let input = input.trim();
            input.split_once(' ').unwrap_or((input, ""))
        };

        match (cmd, args) {
            ("echo", message) => println!("{}", message),
            ("exit", "0") => std::process::exit(0),
            ("type", "echo" | "exit" | "type") => println!("{args} is a shell builtin"),
            ("type", unknown_cmd) => println!("{unknown_cmd}: not found"),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
