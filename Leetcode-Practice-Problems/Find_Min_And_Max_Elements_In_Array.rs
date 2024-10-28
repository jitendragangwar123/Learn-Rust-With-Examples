fn smallest_largest(numbers: &[i32]) -> Option<(i32, i32)> {
    if numbers.is_empty() {
        return None;
    }

    let mut smallest=i32::MAX;
    let mut largest=i32::MIN;

    for &num in numbers {
        if num < smallest {
            smallest = num;
        }
        if num > largest {
            largest = num;
        }
    }
    Some((smallest, largest))
}

fn main() {
    let numbers = [3, 1, 5, 2, 4,7,5,24,56,3,14,-1];
    match smallest_largest(&numbers) {
        Some((smallest, largest)) => println!("The smallest and largest numbers are: {} and {}", smallest, largest),
        None => println!("Array is empty."),
    }
}
