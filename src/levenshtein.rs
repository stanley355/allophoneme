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
