fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}

fn main() {
    let string1 = "Hello";
    let char1 = 'l';
    let result = find_char(string1, char1);
    match result {
        Some(index) => println!("Character '{}' found at index {}", char1, index),
        None => println!("Character '{}' not found in '{}'", char1, string1),
    }
}
