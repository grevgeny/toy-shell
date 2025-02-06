use crate::{
    command::{Command, CommandType},
    tokenizer::Tokenizer,
};

#[derive(Default, Debug)]
pub struct Exit {
    code: i32,
}

impl Command<'_> for Exit {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, mut tokens: Tokenizer) -> Result<(), anyhow::Error> {
        let Some(code_str) = tokens.next() else {
            return Ok(());
        };
        self.code = code_str.parse().unwrap_or(0);
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        std::process::exit(self.code)
    }
}
