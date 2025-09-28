use std::io;
fn main() {
    let mut input_num= String::new();
    io::stdin().read_line(&mut input_num).expect("Eror");
    let s:i32 = input_num.trim().parse().unwrap();
    match s{
        1=>println!("Monday"),
        2=>println!("Tuesday"),
        3=>println!("Wednesday"),
        4=>println!("Thursday"),
        5=>println!("Friday"),
        6=>println!("Saturday"),
        7=>println!("Sunday"),
        _=>print!("inavlid input"),
    };
}

