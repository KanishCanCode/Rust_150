use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    println!("Guess the number !");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    //println!("the secret number is {}",secret_no);
    println!("Please input your guess");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    };
   
    match guess.cmp(&secret_no){
        Ordering::Equal=>println!("You win ,the guess is {}",guess),
        Ordering::Greater=>println!("Too big"),
        Ordering::Less=>println!("to small"),
    }
 println!("your guess : {}",guess);
}