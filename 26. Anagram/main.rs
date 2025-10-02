fn are_anagram(s1:&str,s2:&str)->bool{
    let mut s1_chars:Vec<char>=s1.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    let mut s2_chars:Vec<char>=s2.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    if s1_chars.len()!=s2_chars.len(){
    return false;
}
s1_chars.sort();
s2_chars.sort();

s1_chars==s2_chars
}

fn main(){
    let s1="listen";
    let s2="silent";
    println!("are {} and {} anagram? {}",s1,s2,are_anagram(s1, s2));
}