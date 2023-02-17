use crate::cli::WELCOME_TEXTS;
use crate::excel::Excel;

#[derive(Debug)]
pub struct WordIpaPair {
    pub word: String,
    pub ipa: String,
}

impl WordIpaPair {
    pub fn new(word: String, ipa: String) -> Self {
        Self { word, ipa }
    }

    pub fn encode_ipa_cli() {
        println!("You chose {}", WELCOME_TEXTS[4]);
        println!("Which excel file you want me to read?");
        let excel_files = Excel::find_excel_file_in_parent_dir();
        if excel_files.len() > 0 {
            for (i, excel) in excel_files.iter().enumerate() {
                println!("{}. {}", i + 1, excel);
            }
        }

        let selected_workbook = Excel::request_workbook_input_from_existing_workbooks(&excel_files);

        println!("Which sheet is the dataset sheet?");
        let selected_sheet = Excel::request_sheet_input_from_workbook(&selected_workbook);

        println!(
            "I will read '{1}' sheet from '{0}' workbook:",
            selected_workbook, selected_sheet
        );
        let excel = Excel::new(selected_workbook, selected_sheet);
        let ws_data = excel.fetch_worksheet_data();

        let c = ws_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()));
        println!("{:?}", c);
    }
}
