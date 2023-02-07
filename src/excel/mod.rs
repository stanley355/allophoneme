use crate::cli::Cli;
use calamine::{open_workbook, Reader, Xlsx};

pub struct Excel {
    pub path: String,
    pub sheet: String,
}

impl Excel {
    pub fn request_file() {
        let path = Cli::request_input("Enter file path:");
        let sheet = Cli::request_input("Enter file sheet:");
    }

    pub fn read() {
        let path = "../COC.xlsx";
        let mut excel: Xlsx<_> = open_workbook(path).unwrap();
        if let Some(Ok(r)) = excel.worksheet_range("COC") {
            for row in r.rows() {
                println!("row={:?}, row[0]={:?}", row, row[0]);
            }
        }
    }
}
