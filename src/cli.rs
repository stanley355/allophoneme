use crate::database::Database;
use crate::excel::Excel;
use std::io;
use std::num::ParseIntError;

pub const WELCOME_TEXTS: [&str; 9] = [
    "",
    "Welcome Boss!",
    "How can I help you?",
    "1. Read Financial Report",
    "2. Setup DB and DB CLI",
    "3. Generate Migration",
    "4. Run Migration",
    "5. Revert Migration",
    "6. Insert postgres"
];
const INPUT_LIMIT: usize = WELCOME_TEXTS.len() - 3;

pub struct Cli;

impl Cli {
    pub fn request_input(text: &str) -> String {
        println!("{}", text);
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    pub fn start_menu() {
        for text in WELCOME_TEXTS {
            println!("{}", text);
        }

        let number_input = Self::request_input("Enter a number: ");
        let input_res = number_input.trim().parse::<i32>();
        Self::menu_pick_validation(input_res);
    }

    pub fn menu_pick_validation(input_res: Result<i32, ParseIntError>) {
        match input_res {
            Ok(res) => match res {
                1 => Excel::read_financial_report_cli(),
                2 => Database::setup_db_cli(),
                3 => Database::generate_migration(),
                4 => Database::run_migration(),
                5 => Database::revert_migration(),
                6 => Excel::insert_postgres_data(),
                _ => {
                    eprintln!("Error: Max Input limit is {}!", INPUT_LIMIT);
                    Self::start_menu();
                }
            },
            Err(_) => {
                eprintln!("Error: Input invalid!");
                Self::start_menu();
            }
        }
    }
}
