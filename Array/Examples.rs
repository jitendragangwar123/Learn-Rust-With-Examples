// pass by value
fn main() {
  let array = [1,2,3,4];
  change_array(array); 
  print!("Original array {:?}",array);
}

fn change_array(mut array:[i32;4]){
   for i in 0..4 {
     array[i] = 0;
   }
  println!("Changed array {:?}",array);
}
/*
Output:
Changed array [0, 0, 0, 0]
Original array [1, 2, 3, 4]
*/

//Pass by reference
#![allow(unused)]
fn main() {
  let mut array = [1,2,3,4,5];
  change_array(&mut array);
  print!("Original array {:?}",array);
}
fn change_array(array:&mut [i32;5]){
  for i in 0..5 {
  	array[i] = 0;
  }
 println!("Changed array {:?}",array);
}

/*
Changed array [0, 0, 0, 0, 0]
Original array [0, 0, 0, 0, 0]
*/


// vector in rust
fn main() {
   let v : Vec<i64> = Vec::new(); 
  println!("{ }",v.len());
}

// indexing in vector
fn main() {
   let v = vec!['G','E','E','K','S'];
   let index: usize = 3;
   let ch: char = v[index];
  print!("{ }\n",ch);
}
