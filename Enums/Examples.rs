//We can define the types of the enum's values 
#[derive(Debug)]
enum IpAddressTypes{
    v4(String),
    v6(String),
}

pub fn main(){
   let home=IpAddressTypes::v4(String::from("127.0.0.1"));
   println!("{:?}",home);

   let loopback=IpAddressTypes::v6(String::from("::1"));
   println!("{:?}",loopback);
}
