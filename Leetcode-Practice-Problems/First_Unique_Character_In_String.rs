use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_counts = HashMap::new();
        
        for c in s.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        
        for (i, c) in s.chars().enumerate() {
            if let Some(&count) = char_counts.get(&c) {
                if count == 1 {
                    return i as i32;
                }
            }
        }
     -1
    }
}
