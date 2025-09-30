fn fibbo(n:u32)->u32{
    if n==0{
        0
    }
    else if n==1{
        1
    }
    else{
        fibbo(n-1)+fibbo(n-2)
    }
}
fn main(){
    let n=10;
    println!("The {}th fibonacci number is :{}",n,fibbo(n));
}