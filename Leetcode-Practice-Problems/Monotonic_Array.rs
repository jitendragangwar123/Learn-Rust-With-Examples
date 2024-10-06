impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {return true}
        if nums[0] > nums[nums.len()-1] {
            for i in 1..nums.len() {
                if nums[i] > nums[i-1] {return false}
            }
            return true
        }
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {return false}
        }
        true
    }      
}
