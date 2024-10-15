impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![1]];
        for _ in 1..num_rows {
            let last_row = res.last().unwrap();
            let mut temp = vec![0];
            temp.extend(last_row);
            temp.push(0);
            let mut row = Vec::new();
            for j in 0..last_row.len() + 1 {
                row.push(temp[j] + temp[j + 1]);
            }
            res.push(row);
        }
        res
    }
}
