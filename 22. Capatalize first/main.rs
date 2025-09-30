fn capatalize_words(s:&str)->String{
    let mut result = String::new();
    let mut is_first = true;
    for c in s.chars(){
        if c.is_whitespace(){
            result.push(c);
            is_first=true;
        }else if is_first{
            result.push(c.to_uppercase().next().unwrap());
            is_first = false;
        }else{
            result.push(c);
        }
    }
result}


fn main() {
    let input = "hello ji";
    let result= capatalize_words(input);
    println!("{}",result);
}
