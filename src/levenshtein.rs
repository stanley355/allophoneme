fn levenshtein_distance(s: &str, t: &str) -> usize {
    let n = s.chars().count();
    let m = t.chars().count();

    if n == 0 {
        return m;
    } else if m == 0 {
        return n;
    }

    let mut distance = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        distance[i][0] = i;
    }

    for j in 1..=m {
        distance[0][j] = j;
    }

    for j in 1..=m {
        for i in 1..=n {
            let substitution_cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                0
            } else {
                1
            };

            distance[i][j] = std::cmp::min(
                std::cmp::min(
                    distance[i - 1][j] + 1,
                    distance[i][j - 1] + 1,
                ),
                distance[i - 1][j - 1] + substitution_cost,
            );
        }
    }

    distance[n][m]
}