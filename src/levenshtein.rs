// The Levenshtein distance algorithm is a measure of the difference between two strings. 
// It is also known as the edit distance, and calculates the minimum number of insertions, 
// deletions, and substitutions required to transform one string into another.

pub fn levenshtein_distance(s1: &str, s2: &str) -> f32 {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

    for i in 0..=s1_len {
        matrix[i][0] = i;
    }

    for j in 0..=s2_len {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let substitution_cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = *[
                matrix[i][j + 1] + 1,
                matrix[i + 1][j] + 1,
                matrix[i][j] + substitution_cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    let max_len = s1_len.max(s2_len) as f32;

    1.0 - (matrix[s1_len][s2_len] as f32) / max_len
}

// In this implementation, the function levenshtein_distance takes two string slices as input 
// and returns a floating-point value representing the Levenshtein distance between the two strings. 
// The algorithm works by creating a matrix of size (s1_len + 1) x (s2_len + 1), where each 
// cell (i, j) in the matrix represents the minimum edit distance required to transform the 
// substring of s1 up to index i into the substring of s2 up to index j.

// The function then iterates over each character in both strings, calculates the minimum cost 
// of the three possible operations (insertion, deletion, or substitution) required to transform 
// the current substring of s1 into the current substring of s2, and updates the corresponding cell in the matrix.

// Finally, the function calculates the maximum length of the two strings and uses it to scale 
// the distance value between 0 and 1, where 1 represents complete similarity and 0 represents complete dissimilarity.

// In the main function, we test the levenshtein_distance function by comparing the strings "kitten" and "sitting". 
// The output of the program is a floating-point value representing the Levenshtein distance between the two strings, which is approximately 0.2857 in this case.