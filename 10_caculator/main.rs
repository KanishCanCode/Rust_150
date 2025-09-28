use std::io;
fn main(){
    println!(" Simple calculator");
    println!("Enter an expression");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let tokens:Vec<&str> = input.trim().split_whitespace().collect();
if tokens.len()!=3{
    println!("Invalid input > Format <number> <operator> <number>");
    return;
}
let num1:f64=match tokens[0].parse(){
    Ok(n)=>n,
    Err(_)=>{
        println!("Inalid number");
        return;
    }
};
let operator = tokens[1];
let num2:f64=match tokens[2].parse(){
    Ok(n)=>n,
    Err(_)=>{
        println!("Inalid 2 number");
        return;
    }
};

let result = match operator{
    "+"=>num1+num2,
    "-"=>num1-num2,
    "*"=>num1*num2,
    "/"=>{
        if num2==0.0{
            println!("Error divion by zero");
        }
        num1/num2
    }
    _=>{
    println!("Unsupported operator : {}",operator);
    return;
}
};
println!("REsult is {}",result);
}