use crate::command::{command::CommandType, Command};

#[derive(Default)]
pub struct Echo {
    message: String,
}

impl Command for Echo {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error> {
        self.message = tokens.join(" ");
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        println!("{}", self.message);
        Ok(())
    }
}
