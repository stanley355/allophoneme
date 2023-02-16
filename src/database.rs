use crate::cli::{Cli, WELCOME_TEXTS};
use std::process::Command;
use dotenv::dotenv;
use std::env;

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

    pub fn generate_migration() {
        println!("You chose {}", WELCOME_TEXTS[5]);

        let migration_name = Cli::request_input("What's the name of the migration?");
        let formatted_name = migration_name.to_lowercase().replace(" ", "_");

        let generate_migration_output = Command::new("sqlx")
            .args(["migrate", "add", "-r", &formatted_name])
            .output()
            .expect("Failed to execute process");
        println!("Generate Migration: {:?}", generate_migration_output);
    }

    pub fn run_migration() {
        println!("You chose {}", WELCOME_TEXTS[6]);

        let run_migration_output = Command::new("sqlx")
            .args(["migrate", "run"])
            .output()
            .expect("Failed to execute process");
        println!("Run Migration: {:?}", run_migration_output);
    }
    
    pub fn revert_migration() {
        println!("You chose {}", WELCOME_TEXTS[7]);

        let revert_migration_output = Command::new("sqlx")
            .args(["migrate", "revert"])
            .output()
            .expect("Failed to execute process");
        println!("Revert Migration: {:?}", revert_migration_output);
    }
}
