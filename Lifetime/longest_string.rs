fn longest_string<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn main(){
    let string1=String::from("Hello");
    let result;
    {
        let string2=String::from("hii");
        result=longest_string(string1.as_str(),string2.as_str());
        println!("Result: {}",result);
    }// string2 goes out of the scope
    // println!("Result: {}",result);
}
