use std::collections::HashMap;

fn main() {
    let s= "abcdefghijklmopqrstuvwxyz";
    if s.is_empty(){
        println!("?");
        return;
    }
    let mut freq:HashMap<char,usize>=HashMap::new();

    for c in s.chars(){
        *freq.entry(c).or_insert(0)+=1;
    }
    for (ch,count) in &freq{
        println!("{} : {}",ch,count);
    }
}
