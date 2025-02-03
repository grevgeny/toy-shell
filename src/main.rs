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
            ("exit", "0") => std::process::exit(0),
            ("echo", message) => println!("{}", message),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
