use crate::database::Database;
use crate::excel::Excel;
use std::io;
use std::num::ParseIntError;

const WELCOME_TEXTS: [&str; 7] = [
    "",
    "Welcome Boss!",
    "How can I help you?",
    "1. Read Financial Report",
    "2. Setup Diesel and DB",
    "3. Run Migration",
    "4. Revert Migration"
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
                2 => Database::setup_diesel(),
                3 => Database::run_migration(),
                4 => Database::revert_migration(),
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
