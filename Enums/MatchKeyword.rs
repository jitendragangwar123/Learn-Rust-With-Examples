// match keyword is used to compare values against a series of patterns.
//Example:1
#[allow(dead_code)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

fn move_player(direction:Direction){
    match direction{
        Direction::Up=>println!("Moving player up"),
        Direction::Right=>println!("Moving player right"),
        Direction::Down=>println!("Moving player down"),
        Direction::Left=>println!("Moving player left"),
    };
}

pub fn main(){
    let player_direction=Direction::Up;
    move_player(player_direction);
}

//Example:2
#[allow(dead_code)]
enum Coin{
    Penny,
    Nickel,
    Dine,
    Quarter(Rarity),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Rarity{
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny=>1,
       Coin::Nickel=>5,
       Coin::Dine=>10,
       Coin::Quarter(rarity)=>25,
    }
}

pub fn main(){
    let coin_value=Coin::Quarter(Rarity::Common);
    println!("{}",value_in_cents(coin_value));
    
}
