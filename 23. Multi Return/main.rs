fn calculate_stats(numbers:&[i32])->(i32,i32,f64){
    let sum:i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    let average= if count>0{
        sum as f64 / count as f64 
    }else{ 0.0 };
    (sum,count,average)
}


fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,9,12,123,44];
    let (sum,count,average)=calculate_stats(&numbers);

    println!("Sum :{} ,Count : {}, Average: {:.1}",sum,count,average);
}
