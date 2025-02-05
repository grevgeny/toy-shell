use std::path::PathBuf;

use anyhow;

pub trait Command {
    fn command_type(&self) -> CommandType;
    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error>;
    fn execute(&self) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub enum CommandType {
    Builtin,
    Executable(PathBuf),
}
