use std::io::{self, Write};
use crate::create_command::create_command;

pub fn init() {
    let mut git = create_command("git");
    git.arg("init");

    let status = git.output().unwrap();

    println!("{}", String::from_utf8(status.stdout).unwrap());
    loop {
        print!("Voulez-vous le lier avec une répositorie sur Github? [oui/non]");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        let _ = io::stdin()
            .read_line(&mut user_input)
            .expect("Erreur pendant la saisie");
        match user_input.to_lowercase().trim() {
            "oui" | "o" => {
                println!("");
                print!("Veuillez entrer votre répositorie: ");
                io::stdout().flush().unwrap();
                let mut repo = String::new();
                let _ = io::stdin().read_line(&mut repo).unwrap();
                // let string_command = format!("remote add origin {}", repo.trim());
                let mut git = create_command("git");
                git.args(["remote", "add", "origin", repo.trim()]);
                // println!("{:?}", git.get_args());
                let _ = git.output().unwrap();
                let mut git = create_command("git");
                git.args(["remote", "-v"]);
                let status = git.output().unwrap();
                println!("");
                println!(
                    "Repo {}",
                    String::from_utf8(status.stdout).expect("Erreur d'execution")
                );
                break;
            }
            "non" | "n" => {
                println!("{}", user_input);
                break;
            }
            _ => {
                println!("Réessaie...");
            }
        }
    }
}
