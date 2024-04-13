// Result-Types 
use std::num::ParseIntError; 

fn main() -> Result<(), ParseIntError> { 
	let number_to_str = "GeeksforGeeks"; 
	let num = match number_to_str.parse::<i32>() { 
		Ok(num) => num, 
		Err(e) => return Err(e), 
	}; 
	println!("{}", num); 
	Ok(()) 
} 
