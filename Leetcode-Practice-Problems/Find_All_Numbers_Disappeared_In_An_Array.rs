use std::collections::HashSet;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let new_nums:HashSet<i32>=nums.iter().cloned().collect();
        let mut temp=Vec::new();
        for i in 1..=nums.len() as i32{
            if !new_nums.contains(&i){
                temp.push(i);
            }
        }
        temp
    }
}
