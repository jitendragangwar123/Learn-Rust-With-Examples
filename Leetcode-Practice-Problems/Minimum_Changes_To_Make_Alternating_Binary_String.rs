use std::cmp::min;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut res1 = 0; 
        let mut res2 = 0; 
        
        for (i, c) in s.chars().enumerate() {
            if (i % 2 == 0 && c == '1') || (i % 2 == 1 && c == '0') {
                res1 += 1;
            }
            if (i % 2 == 0 && c == '0') || (i % 2 == 1 && c == '1') {
                res2 += 1;
            }
        }
        min(res1, res2)
    }
}
