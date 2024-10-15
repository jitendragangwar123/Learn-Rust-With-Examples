impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let close_to_open = vec![(')', '('), ('}', '{'), (']', '[')]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>();
        
        for c in s.chars() {
            if let Some(&open) = close_to_open.get(&c) {
                if stack.last() == Some(&open) {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        
        stack.is_empty()
    }
}
