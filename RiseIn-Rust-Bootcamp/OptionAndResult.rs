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

/*
Introduction to Result:
        1.Result is another essential enum in Rust, used for handling errors and returning meaningful information about the cause of the error.
        2.It is a generic enum that can hold two types T and E. 
        3.Result has two variants:
                Ok(T): Represents a successful operation with a value of type T
                Err(E): Represents an error with a value of type E

*/

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

/*
Example of using Result:
*/
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    match division_result {
        Ok(value) => println!("{} divided by {} is: {}", a, b, value),
        Err(error_message) => println!("Error: {}", error_message),
    }
}
