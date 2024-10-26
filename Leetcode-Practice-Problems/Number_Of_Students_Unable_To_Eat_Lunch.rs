use std::collections::HashMap;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut res = students.len() as i32;
        let mut count: HashMap<i32, i32> = HashMap::new();

        for stu in students {
            *count.entry(stu).or_insert(0) += 1;
        }

        for &s in sandwiches.iter() {
            if let Some(c) = count.get_mut(&s) {
                if *c > 0 {
                    *c -= 1;
                    res -= 1;
                } else {
                    return res;
                }
            } else {
                return res;
            }
        }
        res
    }
}
