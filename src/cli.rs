
use crate::excel::Excel;
use std::io;
use std::num::ParseIntError;

const INPUT_LIMIT: i32 = 2;

pub struct Cli;

impl Cli {
    pub fn request_input(text: &str) -> String {
        println!("{}", text);
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        return input.trim().to_string();
    }

    pub fn start_menu() {
        const WELCOME_TEXTS: [&str; 5] = [
            "",
            "Welcome Boss!",
            "How can I help you?",
            "1. Read Excel",
            "2. Migrate Excel",
        ];
        for text in WELCOME_TEXTS {
            println!("{}", text);
        }
        let number_input = Self::request_input("Enter a number: ");
        let input_res = number_input.trim().parse::<i32>();
        Self::menu_pick_validation(input_res);
    }

    pub fn menu_pick_validation(input_res: Result<i32, ParseIntError>) {
        match input_res {
            Ok(res) => {
                match res {
                    1 => {
                        println!("You chose 1. Read Excel");
                        let excel_file = Excel::new();
                        excel_file.read();
                    },
                    2 => println!("Warning: Inactive function"),
                    _ => {
                        eprintln!("Error: Max Input limit is {}!", INPUT_LIMIT);
                        Self::start_menu();
                    }
                }
            }
            Err(_) => {
                eprintln!("Error: Input must be number!");
                Self::start_menu();
            }
        }
    }
}
