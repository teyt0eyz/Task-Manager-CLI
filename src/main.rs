use std::io::{self,Write};

struct Task{
    title:String,
    done:bool,
}

fn main() {
    let mut tasklist = Vec::new();
    
    loop {
        show_manu();
        let choice = get_input("[Menu] select: ");

        match choice.trim() {
            "1" => add_task(&mut tasklist),
            "2" => show_task(&mut tasklist),
            "3" => complete_task(&mut tasklist) ,
            "4" => delete_task(&mut tasklist),
            "5" => {
                println!("Closing.. program..\nGoodbye!.");
                break;
            } ,
            _ => {
                println!("\nInvalid Choice.")
            },
        }
    }
}

fn show_manu(){
    println!("--- Task Manager Menu ---");
    println!("|      1. Add Task      |");
    println!("|      2. Show Task     |");
    println!("|      3. Complete Task |");
    println!("|      4. Delete Task   |");
    println!("|      5. Exit          |");
    println!("-------------------------\n");

}
fn add_task(tasklist:&mut Vec<Task>){
    let name = get_input("[Add] name task: ");

    let cleanname = name.trim().to_string();
    let new_task = Task{
        title: cleanname,
        done: false,
    };

    tasklist.push(new_task);
    println!("Add success!.\n");
}

fn show_task(tasklist:&mut Vec<Task>){
    println!("\n------ All Tasks ------\n");
        if tasklist.is_empty(){
            println!("No tasks yet.");
        }else {
            for (i,task) in tasklist.iter().enumerate(){
                let is_check =  if task.done {"✓"} else {" "};            
                println!("{}.[{}] {}",i+1,is_check,task.title);
            }
        }
    println!("\n------------------------\n");
}

fn complete_task(tasklist:&mut Vec<Task>){
    let choose = get_input("[Complete] select: ");  
    if let Ok(index) = choose.trim().parse::<usize>(){
        if index > 0 && index <= tasklist.len(){
            tasklist[index-1].done = true;
        }    
    }
}

fn delete_task(tasklist:&mut Vec<Task>){
    let delete = get_input("[Delete] select: ");

    if let Ok(index) = delete.trim().parse::<usize>(){
        tasklist.remove(index-1);
    }
}

fn get_input(show : &str) -> String{
    print!("{}",show);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
