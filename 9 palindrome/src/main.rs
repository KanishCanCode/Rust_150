fn is_palindrome(s:&str)->bool{
    let chars:Vec<char>=s.chars().collect();
    let mut left =0;
    let mut right =chars.len().saturating_sub(1);

    while left<right{
        if chars[left]!=chars[right]{
            return false;
        }
        left +=1;
        right -=1;
    }
    true

}
fn main(){
    let s1="racecar";
    let s2="cheese";

    println!("{}->{}",s1,is_palindrome(s1));
    print!("{}->{}",s2,is_palindrome(s2));
}