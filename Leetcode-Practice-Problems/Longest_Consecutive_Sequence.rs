use std::collections::HashSet;
use core::cmp::max;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let new_nums: HashSet<i32> = nums.into_iter().collect();
        let mut longest_cs = 0;

        for &num in &new_nums {
            if !new_nums.contains(&(num - 1)) {
                let mut length = 0;
                while new_nums.contains(&(num + length)) {
                    length += 1;
                }
                longest_cs = max(longest_cs, length);
            }
        }
        longest_cs
    }
}
