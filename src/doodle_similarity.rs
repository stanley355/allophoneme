use crate::{doodle::Doodle, levenshtein::levenshtein_distance};

#[derive(Debug, Clone)]
pub struct DoodleSimilarity {
    pub word: String,
    pub ipa_similarity: f32,
}

impl DoodleSimilarity {
    fn create_doodle_similarity_list(
        target_doodle: &Doodle,
        doodle_list: &Vec<Doodle>,
    ) -> Vec<DoodleSimilarity> {
        let doodle_similariy_list: Vec<DoodleSimilarity> = doodle_list
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

        doodle_similariy_list
    }

    pub fn print_doodle_similarity_list(doodle_list: Vec<Doodle>) {
        for (i, doodle) in doodle_list.clone().iter().enumerate() {
            let doodle_similarity_list = Self::create_doodle_similarity_list(doodle, &doodle_list);
            if doodle_similarity_list.len() > 0 {
                println!("{}. Word: {}", i + 1, doodle.word);
                for doodle_similarity in doodle_similarity_list {
                    println!("-. {:?}", doodle_similarity)
                }
            }
        }
    }
}
