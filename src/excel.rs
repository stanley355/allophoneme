use crate::cli::{Cli, WELCOME_TEXTS};
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::fs;

#[derive(Debug)]
pub struct Excel {
    pub workbook: String,
    pub sheet: String,
}

impl Excel {
    pub fn new(workbook: String, sheet: String) -> Self {
        Self { workbook, sheet }
    }

    pub fn read_excel_cli() {
        println!("You chose {}", WELCOME_TEXTS[3]);

        println!("Which excel file you want me to read?");
        let excel_files = Self::find_excel_file_in_parent_dir();
        if excel_files.len() > 0 {
            for (i, excel) in excel_files.iter().enumerate() {
                println!("{}. {}", i + 1, excel);
            }
        }

        let selected_workbook = Self::request_workbook_input_from_existing_workbooks(&excel_files);

        println!(
            "Which sheet from {workbook} you want me to read?",
            workbook = selected_workbook
        );
        let selected_sheet = Self::request_sheet_input_from_workbook(&selected_workbook);

        println!(
            "I will read '{1}' sheet from '{0}' workbook:",
            selected_workbook, selected_sheet
        );
        let excel = Excel::new(selected_workbook, selected_sheet);
        excel.read_workbook_sheet();
    }

    fn find_excel_file_in_parent_dir() -> Vec<String> {
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

    fn request_workbook_input_from_existing_workbooks(excel_files: &Vec<String>) -> String {
        let path = Cli::request_input("Enter a number:");
        let path_number = path.parse::<usize>();
        match path_number {
            Ok(num) => {
                // Validate input
                let files_len = excel_files.len();
                if num > files_len {
                    eprintln!("Error: File doesn't exist!");
                    Self::request_workbook_input_from_existing_workbooks(excel_files);
                }

                excel_files[num - 1].clone()
            }
            Err(_) => {
                eprintln!("Error: Input should be a number");
                Self::request_workbook_input_from_existing_workbooks(excel_files);
                "".to_string()
            }
        }
    }

    fn request_sheet_input_from_workbook(workbook_name: &String) -> String {
        let workbook: Xlsx<_> = open_workbook(workbook_name).unwrap();
        let worksheets = workbook.sheet_names();

        for (i, sheet) in worksheets.iter().enumerate() {
            println!("{}. {}", i + 1, sheet);
        }

        let sheet_input = Cli::request_input("Enter sheet number:");
        let sheet_number = sheet_input.parse::<usize>();

        match sheet_number {
            Ok(num) => {
                // Validate input
                let sheets_len = worksheets.len();
                if num > sheets_len {
                    eprintln!("Error: File doesn't exist!");
                    Self::request_sheet_input_from_workbook(workbook_name);
                }

                worksheets[num - 1].clone()
            }
            Err(_) => {
                eprintln!("Error: Input should be a number");
                Self::request_sheet_input_from_workbook(workbook_name);
                "".to_string()
            }
        }
    }

    fn read_workbook_sheet(self) {
        let mut excel: Xlsx<_> = open_workbook(&self.workbook).unwrap();
        if let Some(Ok(r)) = excel.worksheet_range(&self.sheet) {
            for (i, row) in r.rows().into_iter().enumerate() {
                println!(
                    "{}. {:?}",
                    i + 1,
                    Self::convert_excel_row_to_table_format(row)
                );
            }
        }
    }

    fn convert_excel_row_to_table_format(row: &[DataType]) -> Vec<String> {
        row.iter().map(|r| format!("{}", r)).collect()
    }
}
