use std::io;

pub struct Cli;

impl Cli {
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
        println!("Enter a number:");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap(); //Read user input

        let input_res = input.trim().parse::<i32>();

        match input_res {
            Ok(res) => println!("Your input is {}", res),
            Err(_) => {
                println!("Input must be number!");
                Self::welcome_input();
            }
        }
    }
}
