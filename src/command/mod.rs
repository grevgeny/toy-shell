mod builtin;
#[allow(clippy::module_inception)]
mod command;
mod executable;

use std::{env, path::PathBuf};

pub use self::command::{Command, CommandType};

pub fn find_command(name: &str) -> Option<Box<dyn Command>> {
    match name {
        "echo" => Some(Box::new(builtin::Echo::default())),
        "type" => Some(Box::new(builtin::Type::default())),
        "exit" => Some(Box::new(builtin::Exit::default())),
        "pwd" => Some(Box::new(builtin::Pwd)),
        "cd" => Some(Box::new(builtin::Cd::default())),
        _ => match find_executable(name) {
            Some(path) => Some(Box::new(executable::Executable::new(
                name.to_string(),
                path,
            ))),
            None => None,
        },
    }
}

fn find_executable(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
