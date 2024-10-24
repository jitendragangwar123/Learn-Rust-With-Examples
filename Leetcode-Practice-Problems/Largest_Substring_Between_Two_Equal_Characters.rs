use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut char_index=HashMap::new();
        let mut res=-1;

        for (i,c) in s.chars().enumerate(){
            if let Some(&prev_index)=char_index.get(&c){
                res=res.max(i as i32-prev_index as i32-1);
            }
            else{
                char_index.insert(c,i as i32);
            }
        }
        res
    }
}
