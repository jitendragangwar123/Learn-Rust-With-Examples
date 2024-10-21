use core::cmp::max;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut zero=0;
        let mut one = s.chars().filter(|&c| c == '1').count() as i32;
        let mut res=0;

        for (i,c) in s.chars().enumerate(){
            if i == s.len() - 1 {
                break;
            }
            
            if c =='0'{
                zero+=1;
            }
            else{
                one-=1;
            }
            res=max(res,zero+one);
        }   
        res           
    }
}
