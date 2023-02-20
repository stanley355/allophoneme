use crate::{
    cli::{Cli, WELCOME_TEXTS},
    constant::IPA_TUPLES,
    excel::Excel,
};

#[derive(Debug, Clone)]
pub struct Doodle {
    pub word: String,
    pub word_ipa: String,
    pub word_ipa_encoded: String,
}

impl Doodle {
    pub fn new(word: String, word_ipa: String) -> Self {
        Self {
            word,
            word_ipa: word_ipa.clone(),
            word_ipa_encoded: Self::encode_word_ipa(word_ipa),
        }
    }

    fn encode_word_ipa(word_ipa: String) -> String {
        let mut arr_word_ipa: Vec<String> = Vec::new();
        for pair in IPA_TUPLES {
            if word_ipa.contains(pair.0) {
                arr_word_ipa.push(String::from(pair.1).clone())
            }
        }

        arr_word_ipa.join("-")
    }

    pub fn encode_word_ipa_from_excel(workbook: String, sheet: String) -> Vec<Doodle> {
        let excel = Excel::new(workbook, sheet);
        let excel_data = excel.fetch_worksheet_data();

        let doodle_list: Vec<Doodle> = excel_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()))
            .collect();

        doodle_list
    }

    pub fn encode_word_ipa_cli() {
        println!("You chose {}", WELCOME_TEXTS[7]);
        println!("Which excel file you want me to read?");
        let excel_files = Excel::find_excel_file_in_parent_dir();

        if excel_files.len() > 0 {
            for (i, excel) in excel_files.iter().enumerate() {
                println!("{}. {}", i + 1, excel);
            }
        } else {
            println!("File not found, returning to main menu");
            Cli::start_menu();
        }

        let workbook = Excel::request_workbook_input_from_existing_workbooks(&excel_files);

        println!("Which sheet is the dataset?");
        let sheet = Excel::request_sheet_input_from_workbook(&workbook);

        println!("Workbook: '{0}' | Sheet: '{1}' ", workbook, sheet);

        // let ipa_encoding_pair_list =
        //     IpaEncodingPair::get_ipa_encoding_pair_list(selected_workbook.clone());
        // let word_ipa_pair_list = Self::get_word_ipa_pair_list(selected_workbook, selected_sheet);

        // let encoded_list = Self::encode_all_word_ipa(ipa_encoding_pair_list, word_ipa_pair_list);

        // for (i, encoded) in encoded_list.iter().enumerate() {
        //     println!("{}, {:?}", i + 1, encoded);
        // }
    }
}
