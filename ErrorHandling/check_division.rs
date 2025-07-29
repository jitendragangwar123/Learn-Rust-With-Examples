fn divide(x: f64, y: f64) -> Result<f64, String> {
    const EPSILON: f64 = 1e-10; // Small threshold for floating-point comparison
    if y.abs() < EPSILON {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    let result = divide(4.0, 5.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // Test with division by zero
    let result_zero = divide(4.0, 0.0);
    match result_zero {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
