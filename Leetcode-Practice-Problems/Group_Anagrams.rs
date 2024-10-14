use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<[i32;26], Vec<String>> = HashMap::new();
        
        for s in strs {
            let mut count = [0;26];
            for c in s.chars() {
                count[(c as usize) - ('a' as usize)] += 1;
            }
            anagrams.entry(count).or_insert(Vec::new()).push(s);
        }
        anagrams.into_values().collect()
    }
}
