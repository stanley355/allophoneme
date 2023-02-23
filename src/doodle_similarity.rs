use crate::{doodle::Doodle, levenshtein::levenshtein_distance};

#[derive(Debug, Clone)]
pub struct DoodleSimilarity {
    pub word: String,
    pub ipa_similarity: f32,
}

impl DoodleSimilarity {
    pub fn create_similar_doodle_list(
        target_doodle: &Doodle,
        doodle_list: &Vec<Doodle>,
    ) -> Vec<DoodleSimilarity> {
        let similar_doodle_list: Vec<DoodleSimilarity> = doodle_list
            .iter()
            // .filter(|dood| dood.word.len() == target_doodle.word.len())  // Jambi research result is missing 2 results
            .map(|dood| DoodleSimilarity {
                word: dood.word.clone(),
                ipa_similarity: levenshtein_distance(
                    &target_doodle.word_ipa_encoded,
                    &dood.word_ipa_encoded,
                ),
            })
            .filter(|dood_similar| {
                dood_similar.ipa_similarity > 0.85 && dood_similar.ipa_similarity < 0.99
            })
            .collect();

        similar_doodle_list
    }

    pub fn print_doodle_similarity_list(doodle_list: Vec<Doodle>) {
        for (i, doodle) in doodle_list.clone().iter().enumerate() {
            let similar_doodle_list = Self::create_similar_doodle_list(doodle, &doodle_list);
            if similar_doodle_list.len() > 0 {
                println!("{}. Word: {}", i + 1, doodle.word);
                for similar_doodle in similar_doodle_list {
                    println!("-. {:?}", similar_doodle)
                }
            }
        }
    }
}
