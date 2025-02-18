use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut res = Vec::new();

        for &num in &nums {
            if visited.contains(&num) {
                res.push(num);
            } else {
                visited.insert(num);
            }
        }
        res
    }
}

fn main() {
    // Step 1: Read user input
    let mut input = String::new();
    println!("Enter space-separated numbers:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Step 2: Parse input into Vec<i32>
    let nums: Vec<i32> = input
        .trim() // Remove extra whitespace
        .split_whitespace() // Split by spaces
        .filter_map(|s| s.parse().ok()) // Convert to i32, ignoring invalid inputs
        .collect();

    // Step 3: Call find_duplicates function
    let duplicates = Solution::find_duplicates(nums);

    // Step 4: Print the result
    println!("Duplicate numbers: {:?}", duplicates);
}
