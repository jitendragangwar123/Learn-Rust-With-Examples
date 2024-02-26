//Control Flow in rust
pub fn main(){
    //if expression
    //match
    //loop expression
    //while expression
    //for expression
    let number=4;
    if number<5{
        println!("True");
    }else{
        println!("False");
    }
}

//if-else example-1
pub fn main(){
    let condition=true;
    let number=if condition{
        5
    }else{
        8
    };
    println!("Number: {}",number)
}


//example-2
pub fn main(){
    let num=60;
    if num%2==0{
        println!("{} is even",num);
    }else{
        println!("{} is odd",num);
    }
    
    if num>20 {
        println!("{} is greater than 20",num);
    }else{
        println!("{} is less than 20",num);
    }
}

//example-3
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

//example-4
fn main (){
    let a=20;
    let b=15;
    let c=2;
    //&& operator
    if a>b && b>c {
        println!("a is greater than b and b is greater than c!");
    }else{
        println!("condition not matched!");
    }
    // || operator
    if a>b || b>c {
        println!("At least one conditon matched!");
    }else{
        println!("condition not matched!");
    }
}

//while-loop
//Example-1
fn main(){
    let mut counter = 0;
    while counter < 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }
}
//Example-2
fn main(){
    let mut count=10;

    while count >0 {
        println!("Hello world {}",count);
        count-=1;
        std::thread::sleep(std::time::Duration::from_secs(2)); //Sleep form  2 seconds.
    }
}

//for-loop
//Example-1
fn main(){
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        println!("Number is {}", number);
    }
}

//Example-2
fn main(){
    let a=[1,2,1,2,1];
    for i in a.iter(){
        println!("Element : {}",i);
    } 

    let s="HelloWorld";
    for c in s.chars(){
        println!("Character: {}",c);
    }
    
    // print the numbers in range 1-3
    for number in 1..4{
        println!("Number : {}",number);
    }
}

//loop keyword
//Example-1
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

//Example-2
fn main(){
    let mut count=1;
    let res=loop{
        count+=1;
        if count==10 {
            break count*2;
        } 
    };
    println!("The res is :{:?}",res); //20
}

//match keyword
//Example-1
fn main() {
    let num = 5;
    match num {
        1 => println!("The number is one!"),
        2 => println!("The number is two!"),
        3 => println!("The number is three!"),
        _ => println!("The number is something else!"),
    }
}

//Example-2
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

//Example-3
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin:Coin)->u8 {
    match coin{
        Coin::Penny=>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter=>25,
    }
}
fn main (){
    let coin=Coin::Nickel;
    let value=value_in_cents(coin);
    println!("Value of coin: {}",value);
}

//FIZZ BUZZ example
fn main(){
    // range from 1-100
    for number in 1..=100{
        if number%3==0 {
            println!("FIZZ");
        }
        else if  number %5 == 0 {
            println!("BUZZ");
        }
        else if number%5==0  && number%3==0 {
            println!("FIZZ BUZZ");
        }else{
            println!("{}",number);
        }
    }
}
