use  crate::{cli::WELCOME_TEXTS, excel::Excel, word_ipa_pair::WordIpaPair};
pub struct Allophone;

impl Allophone {
    pub fn find_similarity_cli() {
        println!("You chose {}", WELCOME_TEXTS[5]);
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

        let list = WordIpaPair::encode_ipa_from_excel(selected_workbook, selected_sheet);

        for (i, li) in list.iter().enumerate() {
            println!("{}, {:?}", i + 1, li);
        }
    }
}