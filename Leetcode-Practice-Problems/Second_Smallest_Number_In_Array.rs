fn second_smallest(numbers:&[i32])->Option<i32>{
    let mut smallest=None;
    let mut second_smallest=None;
    for &num in numbers{
        if Some(num) < smallest || smallest.is_none(){
            second_smallest=smallest;
            smallest=Some(num);
        }
        
        else if Some(num) < second_smallest && Some(num)!=smallest{
            second_smallest=Some(num);
        }
    }
    second_smallest
}

fn main() {
    let numbers = [3, 1, 5, 2, 4];
    match second_smallest(&numbers) {
        Some(value) => println!("The second smallest number is: {}", value),
        None => println!("Array has less than two unique elements."),
    }
}
