// structs used to to create the custom data types
// Similar to tuples but with named field
// used to create more complex data types
// stucts are immutable by default

#[derive(Debug)]
struct User{
    name:String,
    email:String,
    is_active:bool,
    age:u8
}

pub fn main(){
    let mut user1 = User{
        name:String::from("Jhone"),
        email:String::from("jhone@gmail.com"),
        is_active:true,
        age:68
    };
    println!("{:?}",user1);
    //modify the structs values
    user1.name=String::from("Jay");
    println!("Name: {}",user1.name);
}
