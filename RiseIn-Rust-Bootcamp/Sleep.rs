//Sleep method in rust
fn main() {
    println!("Hello World!");
    //wait for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Hii");
}
