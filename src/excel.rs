use crate::cli::Cli;
use calamine::{open_workbook, Reader, Xlsx};
use std::fs;

#[derive(Debug)]
pub struct Excel {
    pub path: String,
    pub sheet: String,
}

impl Excel {
    pub fn new() -> Self {
        let path = Cli::request_input("Enter file path:");
        let sheet = Cli::request_input("Enter file sheet:");

        Self { path, sheet }
    }

    pub fn read_financial_report_cli() {
        println!("You chose 1. Read Financial Report");
        println!("Excel file listed on parent dir:");
        Excel::find_excel_file_in_parent_dir();
        let path = Cli::request_input("Enter file path:");
    }

    pub fn find_excel_file_in_parent_dir() {
        let parent_dir = fs::read_dir("../").unwrap();
        for (i, dir_entry) in parent_dir.into_iter().enumerate() {
            let file_path = dir_entry.unwrap().path().to_string_lossy().to_string();
            let is_excel_entry = &file_path.contains("xlsx");
            if *is_excel_entry {
                println!("{}. {}", i, file_path);
            }
        }
    }

    pub fn read(self) {
        println!("Sheet name: {:?}", &self);
        let mut excel: Xlsx<_> = open_workbook(self.path).unwrap();
        if let Some(Ok(r)) = excel.worksheet_range(&self.sheet) {
            for row in r.rows() {
                println!("row={:?}, row[0]={:?}", row, row[0]);
            }
        }
    }
}
