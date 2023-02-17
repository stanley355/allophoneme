use crate::excel::Excel;

#[derive(Debug)]
pub struct IpaEncodingPair {
    pub ipa: String,
    pub encoding: String,
}

impl IpaEncodingPair {
    fn new(ipa: String, encoding: String) -> Self {
        Self { ipa, encoding }
    }

    pub fn get_ipa_encoding_pair_list(selected_workbook: String) -> Vec<IpaEncodingPair> {
        let excel_ipa = Excel::new(selected_workbook.clone(), String::from("IPA"));
        let ipa_data = excel_ipa.fetch_worksheet_data();

        let ipa_encode_pair_list: Vec<IpaEncodingPair> = ipa_data
            .into_iter()
            .map(|data| Self::new(data[0].clone(), data[1].clone()))
            .collect();

        ipa_encode_pair_list
    }

    pub fn encode_matching_ipa(
        word_ipa: String,
        ipa_encode_pair_list: Vec<IpaEncodingPair>,
    ) -> String {
        let mut base_word_ipa = word_ipa;
        for pair in ipa_encode_pair_list {
            if base_word_ipa.contains(&pair.ipa) {
                base_word_ipa = base_word_ipa.replace(&pair.ipa, &pair.encoding);
            } else {
                base_word_ipa = base_word_ipa.replace(&pair.ipa, "0");
            }
        }

        base_word_ipa
    }
}
