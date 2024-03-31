// match keyword use to compares values against a series of patterns.
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
