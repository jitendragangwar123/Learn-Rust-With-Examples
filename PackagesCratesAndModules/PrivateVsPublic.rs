// declaring a public module
pub mod network{
    //private function within the public module, used to establish a connection
    fn connect(){ 
        println!("COnnection established!");
    }

    //public function that initiates a network connection
    pub fn initiate_connection(){
        connect(); // calls the private function
        println!("Initiating connection.....");
    }
}

fn main(){
    network::initiate_connection();
    //network::connect(); // we can't access the private function
}

/* 
Output: 
Connection established!
Initiating connection.....
*/
