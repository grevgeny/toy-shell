use std::path::PathBuf;
use std::str;

use crate::tokenizer::Tokenizer;

use super::{Command, CommandType};

#[derive(Debug)]
pub struct Executable<'a> {
    name: &'a str,
    path: PathBuf,
    args: Vec<String>,
}

impl<'a> Executable<'a> {
    pub fn new(name: &'a str, path: PathBuf) -> Self {
        Self {
            name,
            path,
            args: Vec::new(),
        }
    }
}

impl<'a> Command<'a> for Executable<'a> {
    fn command_type(&self) -> CommandType {
        CommandType::Executable(&self.path)
    }

    fn parse_args(&mut self, tokens: Tokenizer<'a>) -> Result<(), anyhow::Error> {
        self.args = tokens.collect::<Vec<_>>();
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        let output = std::process::Command::new(self.name)
            .args(&self.args)
            .output()?
            .stdout;
        let output_str = str::from_utf8(&output)?;
        print!("{output_str}");
        Ok(())
    }
}
