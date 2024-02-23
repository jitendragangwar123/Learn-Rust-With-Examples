/*
-Variables are immutable by default.
-To make a variable mutable, you can use the mut keyword before the variable name.
*/

let x = 42; // immutable variable
x = 10; // error: cannot assign twice to immutable variable

let mut x = 42; // mutable variable
x = 10; // OK!

/*
-Rust also has type inference, which means that the compiler can automatically infer the type of a variable based on its value.
*/
let y = 3.14; // Rust infers the type as f64

/*
-Rust also supports shadowing, which means that you can declare a new variable with the same name as an existing variable in a more nested scope.
*/
let x = 42;
let x = x + 1;

//you aren’t allowed to use the mut keyword with constants, Constants aren’t immutable by default.
fn main() { 
    const N: i32 = 1; 
    println!("num: {}", N); 
}


// multiple variables in a single line 
fn main() { 
  let ( first, middle, last ) = ("geeks", "for","geeks"); 
  println!("{} {} {} is amazing", first, middle, last ); 
}

/*
Shadowing:
        Shadowing is an instance where we can make a variable mutable temporarily.
        Shadowing also allows changing the data type of variable. 
*/
fn main() { 
    // original value of gfg  
    // variable is 100 
    let gfg = 100; 
      
    // gfg variable has value 
    // 100-50 = 50 here 
    // gfg variable got shadowed 
    let gfg = gfg -50; 
      
    // Again gfg variable has value 
    // 50*5 = 250 
    // gfg variable got shadowed again 
    let gfg = gfg * 5; 
  
    println!("The value of gfg is: {}", gfg); 
}


// variable declaration examples
fn main(){
    // initialize the variable
    let x; 
    x=7;
    println!("{}",x); //7
    
    // mutable and immutable variable 
    // let a = 5; // immuatable by default
    // a=7;
    // println!("{}",a) // not working 
    
    let mut a=5;
    a=10;
    println!("{}",a); //10
    
    //shadowing : You can declare a variable with the same name many times but in same scope
    let b=5;
    let b=15;
    println!("{}",b);//15
    
    let c=20;
    let c=c+1;
    println!("{}",c); //21
    
    let d=10;
    //inner scope 
    {
        let d=20; // value not changed
    }
    let d=d+12;
    println!("{}",d); //22
    
    
    let e=10;
    //inner scope 
    {
        let e = e+20; // value not changed
    }
    let e=e+12;
    println!("{}",e); //22
    
    let mut f=10;
    {
        f=20; // value changed
    }
    let f=f+1;
    println!("{}",f);//21
}


// you can change the type of variable along with value[shadowing]
fn main(){
   let x=5;
   let x="hello";
   println!("{}",x); //hello
}

// you can change the value of the variable not type
fn main(){
   let mut x=5;
   x="hello";
   println!("{}",x);
   
   let x=5;
   x="hello";
   println!("{}",x);
}

//const keyword in rust
// const variable initialize with Capital letter along with type
fn main(){
   const MAX_POINTS:i32=100_000;
   println!("{}",MAX_POINTS);
}

// const variable initialize with the value
fn main(){
   const MAX_POINTS:i32;
   MAX_POINTS=100_00;
   println!("{}",MAX_POINTS); // not worked
}
