use crate::cli::Cli;
use calamine::{open_workbook, Reader, Xlsx};
use std::fs;

#[derive(Debug)]
pub struct Excel {
    pub path: String,
    pub sheet: String,
}

impl Excel {
    pub fn new(path: String, sheet: String) -> Self {
        Self { path, sheet }
    }

    pub fn read_financial_report_cli() {
        println!("You chose 1. Read Financial Report");

        println!("Which excel file you want me to read?");
        let excel_files = Self::find_excel_file_in_parent_dir();
        if excel_files.len() > 0 {
            for (i, excel) in excel_files.iter().enumerate() {
                println!("{}. {}", i + 1, excel);
            }
        }

        let selected_file = Self::request_file_input_from_existing_files(&excel_files);

        let mut workbook : Xlsx<_>= open_workbook(selected_file).unwrap();

        let worksheets = workbook.sheet_names();
        println!("{:?}", worksheets);
    }

    pub fn find_excel_file_in_parent_dir() -> Vec<String> {
        let mut excel_files: Vec<String> = Vec::new();
        let parent_dir = fs::read_dir("../").unwrap();
        for dir_entry in parent_dir {
            let file_path = dir_entry.unwrap().path().to_string_lossy().to_string();
            let is_excel_entry = &file_path.contains("xlsx");
            if *is_excel_entry {
                excel_files.push(file_path);
            }
        }

        excel_files
    }

    pub fn request_file_input_from_existing_files(excel_files: &Vec<String>) -> String {
        let path = Cli::request_input("Enter a number:");
        let path_number = path.parse::<usize>();
        match path_number {
            Ok(num) => {
                // Validate input
                let files_len = excel_files.len();
                if num > files_len {
                    eprintln!("Error: File doesn't exist!");
                    Self::request_file_input_from_existing_files(excel_files);
                }

                excel_files[num - 1].clone()
            }
            Err(_) => {
                eprintln!("Error: Input should be a number");
                Self::request_file_input_from_existing_files(excel_files);
                "".to_string()
            }
        }
    }
}
