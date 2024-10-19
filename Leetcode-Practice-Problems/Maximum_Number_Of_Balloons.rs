use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count_text = HashMap::new();
        let balloon = "balloon".to_string();
        let mut count_balloon = HashMap::new();

        // Count frequency of characters in text
        for c in text.chars() {
            *count_text.entry(c).or_insert(0) += 1;
        }

        // Count frequency of characters in "balloon"
        for c in balloon.chars() {
            *count_balloon.entry(c).or_insert(0) += 1;
        }

        let mut res = text.len() as i32;

        // Find the minimum number of times "balloon" can be formed
        for (c, &count) in count_balloon.iter() {
            if let Some(&text_count) = count_text.get(c) {
                res = res.min(text_count / count);
            } else {
                res = 0; 
            }
        }
        res
    }
}
