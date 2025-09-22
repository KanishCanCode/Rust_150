fn main() {
    println!("Hello, world!");
    let v=6;
    println!("{}",fact(v));
}

fn fact(v:u64)->u64{
    let mut u=v;
    let mut ans:u64=1;
    loop{
        ans=ans*u;
        u-=1;
        if u==0{
            break;
        }
    }
ans
}