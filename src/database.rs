use std::process::Command;
use crate::cli::WELCOME_TEXTS;

#[derive(Debug)]

pub struct Database;

impl Database {
    pub fn setup_db_cli() {
        println!("You chose {}", WELCOME_TEXTS[4]);

        let diesel_cli_output = Command::new("cargo")
            .args([
                "install",
                "diesel_cli",
                "--no-default-features",
                "--features",
                "postgres",
            ])
            .output()
            .expect("Failed to execute process");
        println!("Diesel cli: {:?}", diesel_cli_output);

        let diesel_setup_output = Command::new("diesel")
            .arg("setup")
            .output()
            .expect("Failed to execute process");
        println!("Diesel setup: {:?}", diesel_setup_output);
    }

    pub fn run_migration() {
        println!("You chose {}", WELCOME_TEXTS[5]);

        let diesel_cli_output = Command::new("diesel")
            .args([
                "migration",
                "run",
            ])
            .output()
            .expect("Failed to execute process");
        println!("Migration cli: {:?}", diesel_cli_output);
    }

    pub fn revert_migration() {
        println!("You chose {}", WELCOME_TEXTS[6]);

        let diesel_cli_output = Command::new("diesel")
            .args([
                "migration",
                "redo",
            ])
            .output()
            .expect("Failed to execute process");
        println!("Migration cli: {:?}", diesel_cli_output);
    }
}
