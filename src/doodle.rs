use crate::{
    cli::{Cli, WELCOME_TEXTS},
    constant::IPA_TUPLES,
    excel::Excel,
    doodle_similarity::DoodleSimilarity,
};

#[derive(Debug, Clone)]
pub struct Doodle {
    pub word: String,
    pub word_ipa: String,
    pub word_ipa_encoded: String,
}

impl Doodle {
    pub fn new(word: String, word_ipa: String) -> Self {
        let trim_ipa = word_ipa.trim().to_string();

        Self {
            word: word.trim().to_string(),
            word_ipa: trim_ipa.clone(),
            word_ipa_encoded: Self::encode_word_ipa(trim_ipa),
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

    pub fn fetch_doodle_data_from_excel(workbook: String, sheet: String) -> Vec<Doodle> {
        let excel = Excel::new(workbook, sheet);
        let excel_data = excel.fetch_worksheet_data();

        let doodle_list: Vec<Doodle> = excel_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()))
            .collect();

        doodle_list
    }

    pub fn check_encoding_cli() {
        println!("You chose {}", WELCOME_TEXTS[4]);
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

        let doodle_dataset = Self::fetch_doodle_data_from_excel(workbook, sheet);

        for (i, dataset) in doodle_dataset.iter().enumerate() {
            println!("{}. {:?}", i + 1, dataset);
        }
    }

    pub fn check_similarities_cli() {
        println!("You chose {}", WELCOME_TEXTS[5]);
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

        let doodle_dataset = Self::fetch_doodle_data_from_excel(workbook, sheet);

        DoodleSimilarity::print_doodle_similarity_list(doodle_dataset);
    }
}

