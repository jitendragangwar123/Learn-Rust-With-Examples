/*
1. Regular comments which are ignored by the compiler:
  // Line comments which go to the end of the line.
  /* Block comments which go to the closing delimiter. */
*/

/*
2. Doc comments which are parsed into HTML library documentation:
  /// Generate library docs for the following item.
  //! Generate library docs for the enclosing item.
*/

//comments.rs
fn main() {
    /*
     * Block comments
     */
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

//Output:Is `x` 10 or 100? x = 10
