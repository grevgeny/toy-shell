use std::path::PathBuf;

use crate::command::{Command, CommandType};

#[derive(Default)]
pub struct Cd {
    path: PathBuf,
}

impl Command for Cd {
    fn command_type(&self) -> CommandType {
        CommandType::Builtin
    }

    #[allow(deprecated)]
    fn parse_args(&mut self, tokens: Vec<String>) -> Result<(), anyhow::Error> {
        let home_dir = std::env::home_dir().unwrap();

        let path = if let Some(path) = tokens.first() {
            if path.eq("~") {
                home_dir
            } else {
                path.into()
            }
        } else {
            home_dir
        };
        self.path = path;

        Ok(())
    }

    fn execute(&self) -> Result<(), anyhow::Error> {
        if self.path.exists() {
            std::env::set_current_dir(&self.path).map_err(Into::into)
        } else {
            println!("cd: {}: No such file or directory", self.path.display());
            Ok(())
        }
    }
}
