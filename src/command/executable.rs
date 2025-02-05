use std::path::PathBuf;
use std::str;

use super::{Command, CommandType};

#[derive(Default)]
pub struct Executable {
    name: String,
    path: PathBuf,
    args: Vec<String>,
}

impl Executable {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            ..Default::default()
        }
    }
}

impl Command for Executable {
    fn command_type(&self) -> CommandType {
        CommandType::Executable(self.path.clone())
    }

    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error> {
        self.args = tokens;
        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        let output = std::process::Command::new(&self.name)
            .args(&self.args)
            .output()?
            .stdout;
        let output_str = str::from_utf8(&output)?;
        print!("{output_str}");
        Ok(())
    }
}
