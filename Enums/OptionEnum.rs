// In rust there is no "Null" value,instead we have "Option" enum.
/*
enum Option<T>{
    Some(T),
    None,
}
*/

/* 
fn main(){
    let x:i8=5;
    let y:Option<i8>=Some(5);

    let sum=x+y;
    println!("{:?}",sum);    
} 
Output :Error
*/

//solution : unwrap the value of y
fn main(){
    let x:i8=5;
    let y:Option<i8>=Some(5);

    let sum=x+y.unwrap();
    println!("Sum of x and y is : {:?}",sum);    
} 
