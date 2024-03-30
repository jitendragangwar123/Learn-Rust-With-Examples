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
