//Enums are a way to define a type by enumerating its possible wariants.
//Enums contains the data as well.
#[derive(Debug)]
enum IpAddressTypes{
    v4,
    v6,
}

pub fn main(){
    let four=IpAddressTypes::v4;
    let six=IpAddressTypes::v6;
    // println!("four: {:?}",four);
    // println!("six: {:?}",six);
    route(four);
    route(six);
}

fn route(ip_type:IpAddressTypes){
    println!("Route called with: {:?}",ip_type);
}
