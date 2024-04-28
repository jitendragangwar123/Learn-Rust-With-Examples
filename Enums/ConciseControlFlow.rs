// fn main() {
//     let config_max: Option<i32>=Some(100);
//     match config_max{
//         Some(max)=>println!("The max configured to be {}", max),
//         _=>(), // ignore the case where config_max is None
//     }
// }

//concise control flow with if let
fn main(){
    let config_max: Option<i32>=Some(100);
    if let Some(max)=config_max{
        println!("The max configured to be {:?}", max)
    }
}
//output : The max configured to be 100
