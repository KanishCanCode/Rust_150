struct Task{
    description:String,
    completed:bool,
}

impl Task{
fn print_fields(&self){
    println!("task description: {}",self.description);
    println!("complteted : {}", self.completed);
}
}

fn main() {
    let task = Task{
        description:String::from("finish this project asap"),
        completed:false,
    };
    task.print_fields();
}
