use rand::Rng;
fn main(){
    let mut attempts=0;
    loop{
        let num = rand::thread_rng().gen_range(1..=8);
        attempts +=1;
        println!("Attempt : {} Genrated number {}",attempts,num);
        if num == 7{
            println!("found 7 bingooo! , in {} attempts",attempts);
            break;
        }
    }
}