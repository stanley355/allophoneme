use crate::cli::WELCOME_TEXTS;
use std::process::Command;

#[derive(Debug)]

pub struct Database;

impl Database {
    pub fn setup_db_cli() {
        println!("You chose {}", WELCOME_TEXTS[4]);
        println!("Setting up DB CLI...");
        let cli_setup_output = Command::new("cargo")
            .args([
                "install",
                "sqlx-cli",
                "--no-default-features",
                "--features",
                "native-tls,postgres",
            ])
            .output()
            .expect("Failed to execute process");
        println!("DB CLI Setup: {:?}", cli_setup_output);

        println!("Setting up Database...");
        let db_creation_output = Command::new("sqlx")
            .args(["database", "create"])
            .output()
            .expect("Failed to execute process");
        println!("DB Creation Setup: {:?}", db_creation_output);
    }
}
