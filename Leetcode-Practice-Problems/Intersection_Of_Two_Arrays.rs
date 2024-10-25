use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let checked: HashSet<i32> = nums1.into_iter().collect(); 
        let mut res = Vec::new();
        
        for &n in nums2.iter() { 
            if checked.contains(&n) && !res.contains(&n) {
                res.push(n); 
            }
        }
        res
    }
}
