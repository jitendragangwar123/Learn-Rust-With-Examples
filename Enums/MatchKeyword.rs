// match keyword is used to compare values against a series of patterns.
// Example:1
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

fn main(){
    let player_direction=Direction::Up;
    move_player(player_direction);
}

// Example:2
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

//Example:3
fn main(){
    fn plus_one(x:Option<i32>)->Option<i32>{
        match x{
            Some(i)=>Some(i+1),
            None=>None,
        }
    }

    let five=Some(5);
    let six=plus_one(five);
    println!("{:?}",six); //Some(6)

    let none=plus_one(None);
    println!("{:?}",none); //None
}

//Catch-all pattern, _
//Example:4
fn main(){
    let x=9;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Other"),
    }
}

//Example:5
fn main() {
    let gfg = String::from("cp"); 
	// match with operator 
	match &*gfg {
		"ds" => println!("Data Structure"),
		"cp" => println!("Competitive programming"),
		"fg" => println!("FAANG"),
		 _  => println!("not in gfg topics")
	}
}
