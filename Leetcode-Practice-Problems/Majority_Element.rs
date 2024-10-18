impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut count = 0;
        for &num in nums.iter() {
            if count == 0 {
                res = num;
            }
            count += if num == res { 1 } else { -1 };
        }
        res
    }
}
