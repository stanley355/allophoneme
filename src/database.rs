use std::process::Command;

#[derive(Debug)]
pub struct Database;

impl Database {
    pub fn setup_diesel() {
        println!("You chose 2. Diesel and DB Setup");

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
        println!("You chose 3. Run Migration");

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
        println!("You chose 4. Revert Migration");

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
