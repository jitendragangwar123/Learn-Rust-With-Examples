use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut directions = std::collections::HashMap::new();
        directions.insert('N', (0, 1));
        directions.insert('S', (0, -1));
        directions.insert('E', (1, 0));
        directions.insert('W', (-1, 0));

        let mut visited = HashSet::new();
        let (mut x, mut y) = (0, 0);
        
        visited.insert((x, y));

        for d in path.chars() {
            let (dx, dy) = directions[&d];
            x += dx;
            y += dy;
            if visited.contains(&(x, y)) {
                return true;
            }
            visited.insert((x, y));
        }
        false
    }
}
