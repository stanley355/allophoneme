use std::process::Command;

#[derive(Debug)]
pub struct Database;

impl Database {
    pub fn setup_diesel_cli() {
        println!("You chose 2. Diesel and DB Setup");
        Command::new("cargo")
            .args([
                "install diesel_cli",
                "--no-default-features",
                "--features postgres",
            ])
            .output()
            .expect("failed to execute process");
    }
}
