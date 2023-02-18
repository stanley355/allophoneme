use crate::{
    cli::{Cli, WELCOME_TEXTS},
    excel::Excel,
    levenshtein::levenshtein_distance,
    word_ipa_pair::WordIpaPair,
};

#[derive(Debug)]
pub struct Allophoneme {
    pub word: String,
    pub similarity: f32,
}

impl Allophoneme {
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

        let encoded_ipa_list =
            WordIpaPair::encode_ipa_from_excel(selected_workbook, selected_sheet);

        // Self::find_similar_words(encoded_ipa_list);
        Self::find_all_similar_words(encoded_ipa_list);
    }

    fn create_similarity_list(
        word_ipa: WordIpaPair,
        word_ipa_list: Vec<WordIpaPair>,
    ) -> Vec<Allophoneme> {
        word_ipa_list
            .into_iter()
            .map(|pair| Allophoneme {
                word: pair.word,
                similarity: levenshtein_distance(&word_ipa.word_ipa, &pair.word_ipa),
            })
            .collect()
    }

    fn find_similar_words(encoded_ipa_list: Vec<WordIpaPair>) {
        println!("Which word is the target word?");
        for (i, encoded_ipa) in encoded_ipa_list.iter().enumerate() {
            println!("{}. {}", i + 1, encoded_ipa.word);
        }
        let target_word = Cli::request_input("Enter a number: ");
        let target_word_parse = target_word.parse::<usize>();

        match target_word_parse {
            Ok(index) => {
                println!("Your target word is: {}", encoded_ipa_list[index - 1].word);
                println!("Most similar words are: ");
                let new_list = Self::create_similarity_list(
                    encoded_ipa_list[index - 1].clone(),
                    encoded_ipa_list,
                );
                for (i, allophoneme) in new_list.into_iter().enumerate() {
                    if allophoneme.similarity > 0.8 {
                        println!("{}, {:?}", i + 1, allophoneme);
                    }
                }
            }
            Err(_) => {
                eprintln!("Input Invalid!");
                Self::find_similar_words(encoded_ipa_list)
            }
        }
    }

    fn find_all_similar_words(encoded_ipa_list: Vec<WordIpaPair>) {
        for (i, encoded_ipa) in encoded_ipa_list.iter().enumerate() {
            println!("{}. Word : {}", i + 1, encoded_ipa.word.clone());

            let new_list = Self::create_similarity_list(
                encoded_ipa_list[i].clone(),
                encoded_ipa_list.clone(),
            );
            for allophoneme in new_list {
                if allophoneme.similarity > 0.85 && allophoneme.similarity < 1.0 {
                    println!("-. {:?}", allophoneme);
                }
            }
        }
    }

}
