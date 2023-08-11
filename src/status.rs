use crate::{add_commit, create_command::create_command, option::option};
use std::io::{self, Write};

pub fn status() {
    let mut git = create_command("git");
    git.arg("status");

    let status = git.output().unwrap();

    println!("{}", String::from_utf8(status.stdout).unwrap());
    loop {
        let option_bool: &str = option("Voulez-vous faire le commit?");
        match option_bool {
            "ok" => {
                println!("");
                print!("Veuillez entrer message: ");
                io::stdout().flush().unwrap();
                let mut repo = String::new();
                let _ = io::stdin().read_line(&mut repo).unwrap();
                add_commit::add_commit(repo.trim());
                break;
            }
            "no" => {
                break;
            }
            "again" => {
                println!("Réessaie...");
            }
            _ => {
                println!("Réessaie...");
            }
        }
    }
}
