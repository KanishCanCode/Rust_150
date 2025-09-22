fn main() {

    println!("Hello, world!");
    let val="heeelllo";
    let reversed = reverse_string(val);
    println!("{}",reversed);
}

fn reverse_string(s:&str)->String{
    s.chars().rev().collect()
}