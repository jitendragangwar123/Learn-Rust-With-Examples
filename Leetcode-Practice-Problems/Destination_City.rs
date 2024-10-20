use std::collections::HashSet;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut cities = HashSet::new();

        for path in &paths {
            cities.insert(&path[0]);
        }

        for path in &paths {
            if !cities.contains(&path[1]) {
                return path[1].clone(); 
            }
        }

        "".to_string()
    }
}
