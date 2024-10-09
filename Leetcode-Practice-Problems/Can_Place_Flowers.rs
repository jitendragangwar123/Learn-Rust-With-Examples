impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut consecutive = 1;
        flowerbed.iter().for_each(|v| {
            if *v == 0 {
                consecutive += 1;
            } else {
                count += (consecutive - 1) / 2;
                consecutive = 0;
            }
        });

        count += consecutive / 2;
        count >= n
    }
}
