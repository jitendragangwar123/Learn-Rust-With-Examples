fn second_larget(numbers:&[i32])->Option<i32>{
    let mut largest=None;
    let mut second_largest=None;
    for &num in numbers{
        if Some(num)>largest{
            second_largest=largest;
            largest=Some(num);
        }
        
        else if Some(num)>second_largest && Some(num)!=largest{
            second_largest=Some(num);
        }
    }
    second_largest
}

fn main() {
    let numbers = [3, 1, 5, 2, 4];
    match second_larget(&numbers) {
        Some(value) => println!("The second highest number is: {}", value),
        None => println!("Array has less than two unique elements."),
    }
}
