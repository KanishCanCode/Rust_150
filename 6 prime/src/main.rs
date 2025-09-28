fn prime_chk(v:u64)->bool{
    if v<=1{
        return false;
    }
    if v==2{
        return false;
    }
    if v%2==0{
        return false;
    }
    let mut i =3;
    while i*i<=v{
        if v%i==0{
            return false;
        }
        i+=2;
    }
    true
}

fn main(){
    let a= 67;
    print!("{}",prime_chk(a));
}