#[derive(Debug)]
struct Person{
    name:String,
    age:i32,
}
fn main() {
    println!("Hello, world!");
    let person = Person{
        name:String::from("hello"),
        age:77,
    };
    println!("{:?}",person);
}

