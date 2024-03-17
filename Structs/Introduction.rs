// structs used to to create the custom data types
// Similar to tuples but with named field
// used to create more complex data types
// stucts are immutable by default

//Example:1
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

//Example:2
#[derive(Debug)]
struct User{
    name:String,
    email:String,
    is_active:bool,
    age:u8
}

pub fn main(){
    let user1=build_user(String::from("Jay"), String::from("jay@gmail.com"));
    println!("{:?}",user1);
}

fn build_user(name:String,email:String)->User{
    User {
        name,
        email,
        is_active:true,
        age:34
    }
}

//Example:3
#[derive(Debug)]
struct User{
    name:String,
    email:String,
    is_active:bool,
    age:u8
}

pub fn main(){
    let user1 = User{
        name:String::from("Jhone"),
        email:String::from("jhone@gmail.com"),
        is_active:true,
        age:68
    };
    
    //create the new instance
    let user2 = User{
        name:String::from("Jacob"),
        email:user1.email.clone(),//one more copy in heap
        is_active:true,
        age:43
    };
    println!("{:?}",user2);
    println!("{:?}",user1);
}

//tuple structs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main(){
    let black=Color(0,0,0);
    let origin=Point(0,0,0);
    println!("Black: {}, {}, {}",black.0,black.1,black.2);
    println!("Origin: {}, {}, {}",origin.0,origin.1,origin.2);
}
