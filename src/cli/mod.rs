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

        return input;
    }

    pub fn welcome_text() {
        const WELCOME_TEXTS: [&str; 4] = [
            "Welcome Boss!",
            "How can I help you?",
            "1. Read Excel",
            "2. Migrate Excel",
        ];
        for text in WELCOME_TEXTS {
            println!("{}", text);
        }
    }

    pub fn welcome_input() {
        let number_input = Self::request_input("Enter a number: ");
        let input_res = number_input.trim().parse::<i32>();
        Self::welcome_input_validation(input_res);
    }

    pub fn welcome_input_validation(input_res: Result<i32, ParseIntError>) {
        match input_res {
            Ok(res) => {
                println!("Your input is {}", res);

                match res {
                    1 => Excel::request_file(),
                    2 => println!("Inactive function"),
                    _ => {
                        println!("Input limit is: {}", INPUT_LIMIT);
                        Self::welcome_input();
                    }
                }
            }
            Err(_) => {
                println!("Input must be number!");
                Self::welcome_input();
            }
        }
    }
}
