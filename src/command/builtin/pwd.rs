use crate::command::{Command, CommandType};

#[derive(Debug, Default)]
pub struct Pwd;

impl Command for Pwd {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, _tokens: Vec<String>) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        let wd = std::env::current_dir()?;
        println!("{}", wd.display());
        Ok(())
    }
}
