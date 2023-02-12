use crate::cli::Cli;
use calamine::{open_workbook, Reader, Xlsx};

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
