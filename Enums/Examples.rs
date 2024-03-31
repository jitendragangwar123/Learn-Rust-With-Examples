//We can define the types of the enum's values 
//Example:1
#[derive(Debug)]
enum IpAddressTypes{
    v4(String), //String type
    v6(String),
}

pub fn main(){
   let home=IpAddressTypes::v4(String::from("127.0.0.1"));
   println!("{:?}",home);

   let loopback=IpAddressTypes::v6(String::from("::1"));
   println!("{:?}",loopback);
}


//Example:2
#[derive(Debug)]
enum IpAddressTypes{
    v4(u8,u8,u8,u8), //u8 type
    v6(String),
}
fn main(){
   let home=IpAddressTypes::v4(127,0,0,1);
   println!("{:?}",home);

   let loopback=IpAddressTypes::v6(String::from("::1"));
   println!("{:?}",loopback);
}

//Example:3
//Implements a simple system based on Messages
#[derive(Debug)]
enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

//methods of enums
impl Message {
    fn call(&self){
        println!("{:?}",self);
    }
}

fn main(){
    let m=Message::Write(String::from("Hello Jay"));
    let x=Message::Move{x:3,y:8};
    let y=Message::ChangeColor(0, 0, 0);
    let z=Message::Quit;

    m.call();
    x.call();
    y.call();
    z.call();
}
