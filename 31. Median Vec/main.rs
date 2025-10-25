fn find_median(numbers:&mut Vec<f64>)->f64{

    numbers.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let len = numbers.len();
    if len ==0{
        return 0.0;
    }
    if len%2==1{
        numbers[len/2]
    }else{
        (numbers[(len/2)-1]+numbers[len/2])/2.0
    }
}

fn main(){
    let mut vec1=vec![1.0,4.5,20.1,77.90];

    println!("Median of vec1 is {}",find_median(&mut vec1));
}