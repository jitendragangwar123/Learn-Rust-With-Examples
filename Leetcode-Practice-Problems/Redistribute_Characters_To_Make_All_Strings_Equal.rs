use std::collections::HashMap;
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count_char = HashMap::new();

        for word in &words {
            for c in word.chars() {
                *count_char.entry(c).or_insert(0) += 1;
            }
        }

        for &count in count_char.values() {
            if count % words.len() != 0 {
                return false;
            }
        }
        true
    }
}
