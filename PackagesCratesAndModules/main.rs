use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main(){
  let plant:Asparagus=Asparagus{
    name:String::from("Asparagus"),
    stalks:5,
  };
  println!("I'm growing {:?}!",plant);
}
