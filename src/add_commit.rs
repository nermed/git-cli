use crate::{create_command::create_command, option::option, push::push};

pub fn add_commit(message: &str) {
    let mut git = create_command("git");
    git.args(["add", "."]);
    let status = git.output().unwrap();
    println!("{}", String::from_utf8(status.stdout).unwrap());

    let mut git = create_command("git");
    git.args(["commit", "-m", message]);
    let status = git.output().unwrap();
    println!("{}", String::from_utf8(status.stdout).unwrap());
    loop {
        let option_bool: &str = option("Voulez-vous faire un push? [oui/non]");
        match option_bool {
            "ok" => {
                push();
                break;
            }
            "no" => break,
            "again" => {
                println!("Veuillez choisir entre oui et non");
            }
            _ => {
                println!("Veuillez choisir entre oui et non");
            }
        }
    }
}
