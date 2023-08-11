use std::io::{self, Write};

pub fn option(message: &str) -> &str {
    print!("{} [oui/non]", message);
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        let _ = io::stdin()
            .read_line(&mut user_input)
            .expect("Erreur pendant la saisie");
        match user_input.to_lowercase().trim() {
            "oui" | "o" => {
                "ok"
            }
            "non" | "n" => {
                "no"
            }
            _ => {
                "again"
            }
        }
}
