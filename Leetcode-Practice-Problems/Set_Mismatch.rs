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
