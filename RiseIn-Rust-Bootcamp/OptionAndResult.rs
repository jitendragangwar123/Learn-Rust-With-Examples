/*
Introduction to Option:
        1.Option is an enum that represents the possibility of a value being present or absent.
        2.It is a generic enum that can hold any type T.
        3.It has two variants:
          Some(T): Represents a value of type T
          None: Represents the absence of a value
*/

pub enum Option<T> {
    Some(T),
    None,
}

/*
Example of using Option:
*/
fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn main() {
    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is: {}", number, value),
        None => println!("The square root of {} is not a real number.", number),
    }
}
