use std::cmp::min;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut res=0;
        let k=k as usize;
        for (i,&ticket) in tickets.iter().enumerate(){
            if i<=k {
                res+=min(tickets[i],tickets[k]);
            }
            else{
                res+=min(tickets[i],tickets[k]-1);
            }
        }
        res
    }
}
