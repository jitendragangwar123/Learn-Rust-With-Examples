// Option method
// fn main() {
//     let config_max: Option<i32>=Some(100);
//     match config_max{
//         Some(max)=>println!("The max configured to be {}", max),
//         _=>(), // ignore the case where config_max is None
//     }
// }

//Example:1
//concise control flow with if let
fn main(){
    let config_max: Option<i32>=Some(100);
    if let Some(max)=config_max{
        println!("The max configured to be {:?}", max)
    }
}
//output : The max configured to be 100

//Example:2
#![allow(dead_code)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}
#[derive(Debug)]
enum Rarity{
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

fn main(){
    let coin=Coin::Quarter(Rarity::Common);
    if let Coin::Quarter(rarity) = coin {
        println!("This quarter is a {:?}",rarity);
    } else{
        println!("This is not a quarter");
    }
}

//output: This quarter is a Common
