use crate::{
    cli::WELCOME_TEXTS,
    constant::IPA_TUPLES,
    doodle_similarity::{SimilarDoodle, SimilarDoodleCollection},
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

    pub fn create_doodle_data_from_excel(excel: Excel) -> Vec<Doodle> {
        let excel_data = excel.fetch_worksheet_data();

        let doodle_list: Vec<Doodle> = excel_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()))
            .collect();

        doodle_list
    }

    pub fn check_encoding_cli() {
        println!("You chose {}", WELCOME_TEXTS[4]);
        let excel = Excel::read_excel_request();

        let doodle_dataset = Self::create_doodle_data_from_excel(excel);

        for (i, dataset) in doodle_dataset.iter().enumerate() {
            println!("{}. {:?}", i + 1, dataset);
        }
    }

    pub fn check_similarities_cli() {
        println!("You chose {}", WELCOME_TEXTS[5]);
        let excel = Excel::read_excel_request();
        let doodle_dataset = Self::create_doodle_data_from_excel(excel);

        SimilarDoodle::print_similar_doodle_list(doodle_dataset);
    }

    pub fn check_similarities_and_clean_cli() {
        println!("You chose {}", WELCOME_TEXTS[6]);
        let excel = Excel::read_excel_request();
        let doodle_dataset = Self::create_doodle_data_from_excel(excel);

        // let doodle_similarity_list = DoodleSimilarity::create_doodle_similarity_list(target_doodle, doodle_list);
        // DoodleSimilarity::print_doodle_similarity_list(doodle_dataset);
        let similar_doodle_collection = SimilarDoodleCollection::new(doodle_dataset);

        for (i, similar_doodle) in similar_doodle_collection.iter().enumerate() {
            println!("{}. {:?}", i + 1, similar_doodle);
        }
    }
}
