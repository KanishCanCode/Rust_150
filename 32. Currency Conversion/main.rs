use std::io;

fn main() {
    println!("enter the amount of usd");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("cheesy");
let y:i64=input.trim().parse().unwrap();
let  amount = y*88;
print!("usd to inr is {}",amount);
}
