// First Approach:
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
       let mut nums = nums;
       // sort
       for i in 0..nums.len() {
           while (nums[i] != nums[(nums[i]-1) as usize]) {
               let tmp = nums[i];
               nums[i] = nums[(nums[i]-1) as usize];
               nums[(tmp-1) as usize] = tmp;
           }
        }
        for i in 0..nums.len() {
            if nums[i] != ((i+1) as i32) {
                return vec![nums[i], (i+1) as i32];
            }
        }
        return vec![];
    }
}

// Second Approach:
use std::collections::HashMap;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0, 0]; 
        let mut count = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        for i in 1..=nums.len() as i32 {
            match count.get(&i) {
                Some(&2) => res[0] = i,  
                None => res[1] = i,      
                _ => {}
            }
        }
        res
    }
}
