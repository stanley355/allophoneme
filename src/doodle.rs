use crate::constant::IPA_TUPLES;

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
}
