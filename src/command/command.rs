use core::fmt;
use std::path::Path;

use anyhow;

use crate::tokenizer::Tokenizer;

pub trait Command<'a>: fmt::Debug {
    fn command_type(&self) -> CommandType;
    fn parse_args(&mut self, tokens: Tokenizer<'a>) -> Result<(), anyhow::Error>;
    fn execute(&self) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub enum CommandType<'a> {
    Builtin,
    Executable(&'a Path),
}
