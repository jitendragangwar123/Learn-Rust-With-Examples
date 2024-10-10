impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();

        let mut left: i32 = 0;

        for (index, &num) in nums.iter().enumerate() {
            let right: i32 = total - left - num;

            if left == right {
                return index as i32;
            }

            left += num;
        }

        -1
    }
}
