// Generate Random Numbers within a Range 
use rand::Rng; 

fn main() { 
	let mut rng = rand::thread_rng(); 
	// for range
	for _x in 0..5{ 
	  let num: u8 = rng.gen_range(0..10); 
	  println!("Random number between 0 and 9: {}", num); 
	} 
} 

// Generate random numbers Using the inclusive range 
use rand::Rng; 

fn main() { 
	let mut rng = rand::thread_rng(); 
	
	for _x in 0..5{ 
	  let num: u8 = rng.gen_range(0..=10); 
	  println!("Random number between 0 and 10 : {}", num); 
	} 
} 

// Generating random boolean value 
use rand::Rng; 

fn main() { 
	let mut rng = rand::thread_rng(); 
	
	for _x in 0..5{ 
	  let toss: bool = rng.gen_bool(0.5); 
	  println!("Random boolean value : {}", toss); 
	} 
} 

