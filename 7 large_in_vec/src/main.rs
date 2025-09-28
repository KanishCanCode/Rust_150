 fn main(){
    let nums=vec![2,3,4,5,6,7,14,46,545,45];
    match nums.iter().max(){
Some(max)=>println!("The largest element is {}" , max),
None=>println!("vector is empty")
    }
 }