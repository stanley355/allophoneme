use crate::{
    cli::{WELCOME_TEXTS, Cli}, excel::Excel, levenshtein::levenshtein_distance, word_ipa_pair::WordIpaPair,
};

#[derive(Debug)]
pub struct Allophone {
    pub word: String,
    pub similarity: f32,
}

impl Allophone {
    pub fn find_similarity_cli() {
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

        let selected_workbook = Excel::request_workbook_input_from_existing_workbooks(&excel_files);

        println!("Which sheet is the dataset sheet?");
        let selected_sheet = Excel::request_sheet_input_from_workbook(&selected_workbook);

        println!(
            "I will read '{1}' sheet from '{0}' workbook:",
            selected_workbook, selected_sheet
        );

        let list = WordIpaPair::encode_ipa_from_excel(selected_workbook, selected_sheet);

        let new_list = Self::create_similarity_list(list[0].clone(), list);
        println!("Target word: {}", new_list[0].word.clone());
        for (i, li) in new_list.into_iter().enumerate() {
            println!("{}, {:?}", i + 1, li);
        }
    }

    fn create_similarity_list(
        word_ipa: WordIpaPair,
        word_ipa_list: Vec<WordIpaPair>,
    ) -> Vec<Allophone> {
        word_ipa_list
            .into_iter()
            .map(|pair| Allophone {
                word: pair.word,
                similarity: levenshtein_distance(&word_ipa.word_ipa, &pair.word_ipa),
            })
            .collect()
    }
}
