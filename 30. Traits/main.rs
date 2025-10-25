trait Speak{
    fn speak(&self)->String;
}

struct Dog{
    name:String,
}

impl Speak for Dog{
    fn speak(&self)->String{
        format!("{} says : Woof!",self.name)
    }
}

struct Cat{
    name:String,
}

impl Speak for Cat{
    fn speak(&self)->String{
        format!("{} says : Moew!",self.name)
    }
}

fn main(){
    let dog = Dog{name:String::from("tom")};
    let cat = Cat{name:String::from("Pimpy")};
    println!("{}",dog.speak());
    println!("{}",cat.speak());
}