use calamine::{open_workbook, Reader, Xlsx};

pub struct Excel {
    pub path: String,
    pub sheet: String,
}

impl Excel {
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
