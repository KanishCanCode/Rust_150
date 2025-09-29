use std::io;
fn main() {
    let mut tasks:Vec<String>= Vec::new();
    println!("Welcome to cli rodo list");

    loop{
        println!("\n OPtions:");
        println!("1. Add new task");
        println!("2. view task");
        println!("3. DElete task");
        println!("4. Exit");
        println!("Enter your cjoice");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice:usize=match choice.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("invalid input enter 1 to 4");
                continue;
            }
        };
        match choice{
            1=>{
                println!("Enter task description");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read task");
                let task=task.trim();
                if !task.is_empty(){
                    tasks.push(task.to_string());
                    println!("Task added : {}",task);
                }
                else{
                    println!("Task can not be empty");
                }
            }
            2=>{
                if tasks.is_empty(){
                    println!("No tasks in the list");
                }
                else{
                    print!("\n your tasks");
                    for (index,task) in tasks.iter().enumerate(){
                        println!("{}. {}",index+1,task);

                    }
                }
            }
        3=>{
            if tasks.is_empty(){
                println!("Not tasks");
            }else{
                println!("\n your tasks");
                for (index,task) in tasks.iter().enumerate(){
                    println!("{}. {}",index+1,task);
                }
                println!("Enter task number");
                let mut index= String::new();
                match io::stdin().read_line(&mut index){
                    Ok(_)=>{
                        match index.trim().parse::<u32>(){
                            Ok(num) if num > 0 && num <=tasks.len().try_into().unwrap()=>{
                                let removed_task=tasks.remove((num-1).try_into().unwrap());
                                println!("Task deleted : {}",removed_task);
                            }
                            _=>println!("Invalid task number ! please enter a number between 1 and {}",tasks.len()),
                        }
                    }
                Err(_)=>{
                    println!("Faiedl to read input");
                }
                }
            }
        }
        4=>{
            println!("Bye bye broo");
            break;
        }
        _=>println!("invalid choice enter number from 1 to 4"),
        
        }
    }
}
