/*
Testing in rust
*/
#[cfg(test)] 
mod tests { 
  #[test] 
  fn multiply_function() { 
     let result = 5*5; 
     assert_eq!(result, 25); 
  } 
}
