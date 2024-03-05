#![crate_type = "lib"] 
#![crate_name = "gfg"] 
#[allow(dead_code)] 
fn main(){ 
pub fn pub_func() { 
	println!("calling GFG `pub_func()`"); 
} 
fn pvt_func() { 
	println!("calls GFG `pvt_func()`"); 
} 

pub fn new_access() { 
	print!("called GFG `new_access()`, that\n> "); 
pvt_func(); 
} 
    pub_func(); 
    pvt_func(); 
    new_access(); 
} 


//Cargo.toml file
[package]
name = "gfg"
version = "0.1.0"
authors = ["runner"]
edition = "2021"
[dependencies]
