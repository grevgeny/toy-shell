use crate::{
    command::{command::CommandType, Command},
    tokenizer::Tokenizer,
};

#[derive(Default, Debug)]
pub struct Echo {
    message: String,
}

impl Command<'_> for Echo {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    fn parse_args(&mut self, tokens: Tokenizer) -> Result<(), anyhow::Error> {
        self.message = tokens.collect::<Vec<_>>().join(" ");
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        println!("{}", self.message);
        Ok(())
    }
}
