#[allow(dead_code)]
fn distinct_subsequences(s: String, t: String) -> i32 {
    fn go(i: usize, j: usize, dp: &mut Vec<Vec<i32>>, s: &Vec<char>, t: &Vec<char>) -> i32 {
        if j == t.len() {
            return 1;
        }
        if i == s.len() {
            return 0;
        }
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        let leave = go(i + 1, j, dp, s, t);
        let take = if s[i] == t[j] {
            go(i + 1, j + 1, dp, s, t)
        } else {
            0
        };

        dp[i][j] = leave + take;
        dp[i][j]
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let mut dp = vec![vec![-1; t.len()]; s.len()];
    go(0, 0, &mut dp, &s, &t)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_subsequences() {
        assert_eq!(
            distinct_subsequences("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
        assert_eq!(
            distinct_subsequences("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
