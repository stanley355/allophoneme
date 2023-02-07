use calamine::{open_workbook, Reader, Xlsx};

pub struct ExcelReader;

impl ExcelReader {
    pub fn read() {
        let mut excel: Xlsx<_> = open_workbook("../COC.xlsx").unwrap();
        if let Some(Ok(r)) = excel.worksheet_range("COC") {
            for row in r.rows() {
                println!("row={:?}, row[0]={:?}", row, row[0]);
            }
        }
    }
}
