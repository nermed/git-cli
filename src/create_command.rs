use std::process::Command;

pub fn create_command(params: &str) -> Command {
    Command::new(params)
}