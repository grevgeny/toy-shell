use crate::command::{Command, CommandType};

#[derive(Default, Debug)]
pub struct Exit {
    code: i32,
}

impl Command for Exit {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error> {
        let Some(code_str) = tokens.first() else {
            return Ok(());
        };
        self.code = code_str.parse().unwrap_or(0);
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        std::process::exit(self.code)
    }
}
