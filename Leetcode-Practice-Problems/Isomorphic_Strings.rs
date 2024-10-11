use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut hs: HashMap<char, char> = HashMap::with_capacity(s.len());
        for both in s.chars().zip(t.chars()) {
            let old = hs.insert(both.0, both.1);
            if old.is_some() && old.unwrap() != both.1 {
                return false;
            }
        }

        let mut hs: HashMap<char, char> = HashMap::with_capacity(s.len());
        for both in t.chars().zip(s.chars()) {
            let old = hs.insert(both.0, both.1);
            if old.is_some() && old.unwrap() != both.1 {
                return false;
            }
        }

        true
    }
}
