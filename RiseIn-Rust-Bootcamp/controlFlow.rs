//Conditional Statements:

//if-else
fn main(){
  let x = 5;
    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    }
}

//while-loop
fn main(){
    let mut counter = 0;
    while counter < 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }
}

//for-loop
fn main(){
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        println!("Number is {}", number);
    }
}


//loop:keyword
fn main(){
    let mut counter = 0;
    loop {
        println!("Counter value is {}", counter);
        counter += 1;
    
        if counter == 5 {
            break;
        }
    }
}

//match:keyword
fn main() {
    let num = 5;
    match num {
        1 => println!("The number is one!"),
        2 => println!("The number is two!"),
        3 => println!("The number is three!"),
        _ => println!("The number is something else!"),
    }
}

fn main() {
    let num = 2;
    let result = match num {
        1 => "The number is one!",
        2 => "The number is two!",
        3 => "The number is three!",
        _ => "The number is something else!",
    };
   
    println!("{}", result);
}

