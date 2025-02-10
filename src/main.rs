use std::io::{self, Write};

use toy_shell::{command::find_command, tokenizer::Tokenizable};

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }

        let mut tokens = input.tokens();
        let Some(command_name) = tokens.next() else {
            continue;
        };

        let command = if let Some(mut command) = find_command(&command_name) {
            command.parse_args(tokens)?;
            command
        } else {
            println!("{command_name}: command not found");
            continue;
        };

        command.execute()?;
    }
}
