struct Rectangle{
    length:u64,
    breadth:u64,
}
impl Rectangle{
    fn area(&self)->u64{
        self.length*self.breadth
    }
}

fn main(){
    let rect=Rectangle{
        length:58,
        breadth:6,
    };
    println!("area of rectangle is {}",rect.area());
}