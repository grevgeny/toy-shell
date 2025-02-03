use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => std::process::exit(0),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
