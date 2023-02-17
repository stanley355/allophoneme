use crate::cli::WELCOME_TEXTS;
use crate::excel::Excel;
use crate::ipa_encoding_pair::IpaEncodingPair;

#[derive(Debug, Clone)]
pub struct WordIpaPair {
    pub word: String,
    pub word_ipa: String,
}

impl WordIpaPair {
    pub fn new(word: String, word_ipa: String) -> Self {
        Self { word, word_ipa }
    }

    pub fn encode_ipa_cli() {
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

        let selected_workbook = Excel::request_workbook_input_from_existing_workbooks(&excel_files);

        println!("Which sheet is the dataset sheet?");
        let selected_sheet = Excel::request_sheet_input_from_workbook(&selected_workbook);

        println!(
            "I will read '{1}' sheet from '{0}' workbook:",
            selected_workbook, selected_sheet
        );

        let ipa_encoding_pair_list =
            IpaEncodingPair::get_ipa_encoding_pair_list(selected_workbook.clone());
        let word_ipa_pair_list = Self::get_word_ipa_pair_list(selected_workbook, selected_sheet);

        let encoded_list = Self::encode_all_word_ipa(ipa_encoding_pair_list, word_ipa_pair_list);

        for (i, encoded) in encoded_list.iter().enumerate() {
            println!("{}, {:?}", i + 1, encoded);
        }
    }

    pub fn encode_ipa_from_excel(
        selected_workbook: String,
        selected_sheet: String,
    ) -> Vec<WordIpaPair> {
        let ipa_encoding_pair_list =
            IpaEncodingPair::get_ipa_encoding_pair_list(selected_workbook.clone());
        let word_ipa_pair_list = Self::get_word_ipa_pair_list(selected_workbook, selected_sheet);

        Self::encode_all_word_ipa(ipa_encoding_pair_list, word_ipa_pair_list)
    }

    fn get_word_ipa_pair_list(
        selected_workbook: String,
        selected_sheet: String,
    ) -> Vec<WordIpaPair> {
        let excel = Excel::new(selected_workbook, selected_sheet);
        let ws_data = excel.fetch_worksheet_data();

        let word_ipa_pair_list: Vec<WordIpaPair> = ws_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()))
            .collect();

        word_ipa_pair_list
    }

    fn encode_all_word_ipa(
        ipa_encoding_pair_list: Vec<IpaEncodingPair>,
        word_ipa_pair_list: Vec<WordIpaPair>,
    ) -> Vec<WordIpaPair> {
        word_ipa_pair_list
            .into_iter()
            .map(|pair: WordIpaPair| WordIpaPair {
                word: pair.word,
                word_ipa: IpaEncodingPair::encode_matching_ipa(
                    pair.word_ipa,
                    &ipa_encoding_pair_list,
                ),
            })
            .collect()
    }
}
