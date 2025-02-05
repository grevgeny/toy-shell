use crate::command::{find_command, Command, CommandType};

#[derive(Default, Debug)]
pub struct Type {
    cmd_name: String,
}

impl Command for Type {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error> {
        let Some(command_name) = tokens.first() else {
            return Ok(());
        };
        self.cmd_name.clone_from(command_name);
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        if self.cmd_name.is_empty() {
            println!();
            return Ok(());
        }

        match find_command(&self.cmd_name) {
            Some(cmd) => match cmd.command_type() {
                CommandType::Builtin => {
                    println!("{} is a shell builtin", self.cmd_name);
                }
                CommandType::Executable(path) => {
                    println!("{} is {}", self.cmd_name, path.display());
                }
            },
            None => println!("{}: not found", self.cmd_name),
        }
        Ok(())
    }
}
