use crate::{doodle::Doodle, levenshtein::levenshtein_distance};

#[derive(Debug, Clone)]
pub struct DoodleSimilarity {
    pub word: String,
    pub ipa_similarity: f32,
}

impl DoodleSimilarity {
    pub fn find_similar_doodles_from_list(
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

    pub fn print_similar_doodle_list(doodle_list: Vec<Doodle>) {
        for (i, doodle) in doodle_list.clone().iter().enumerate() {
            let similar_doodle_list = Self::find_similar_doodles_from_list(doodle, &doodle_list);
            if similar_doodle_list.len() > 0 {
                println!("{}. Word: {}", i + 1, doodle.word);
                for similar_doodle in similar_doodle_list {
                    println!("-. {:?}", similar_doodle)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimilarDoodleCollection {
    pub word: String,
    pub similar_doodles: Vec<DoodleSimilarity>,
}

impl SimilarDoodleCollection {
    pub fn new(doodles: Vec<Doodle>) -> Vec<SimilarDoodleCollection> {
        let similar_doodle_collection = doodles
            .iter()
            .map(|doodle| SimilarDoodleCollection {
                word: doodle.word.clone(),
                similar_doodles: DoodleSimilarity::find_similar_doodles_from_list(doodle, &doodles),
            })
            .filter(|doodle_collection| doodle_collection.similar_doodles.len() > 0)
            .collect();

        similar_doodle_collection
    }
}
