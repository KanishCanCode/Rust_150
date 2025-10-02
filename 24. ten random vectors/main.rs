use rand::Rng;
fn main() {
    let mut numbers:Vec<i32>= Vec::new();
    let mut i=0;
    loop{
        let s= rand::thread_rng().gen_range(1..=100);
        
        numbers.push(s);
        i+=1;
        if i==10{
            break;
        }
    }
println!("the random vector is {:?}",numbers);
}

