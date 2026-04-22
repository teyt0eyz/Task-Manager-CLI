use std::io::{self,Write,BufRead};
use std::fs::{File};

struct Task{
    title:String,
    done:bool,
}

fn main() {
    let mut tasklist = load_tasks();
    
    loop {
        show_menu();
        let choice = get_input("[Menu] select: ");

        match choice.trim() {
            "1" => {
                add_task(&mut tasklist);
                save(&tasklist);
            },
            "2" => show_task(&tasklist),
            "3" => {
                complete_task(&mut tasklist);
                save(&tasklist);
            } ,
            "4" => {
                 delete_task(&mut tasklist);
                 save(&tasklist);
            },
            "5" => {
                println!("Closing.. program..\nGoodbye!👋");
                break;
            } ,
            _ => {
                println!("\n❌  Invalid Choice.")
            },
        }
    }
}

fn load_tasks() -> Vec<Task>{
    let mut tasks = Vec::new();
    
    let file = match File::open("tasks.txt") {
        Ok(f) => f,
        Err(_) => return tasks,
    };

    let reader  = io::BufReader::new(file) ;
    for line in reader.lines(){
        if let Ok(content) = line{
            let parts: Vec<&str> = content.split('|').collect();
            if parts.len() == 2{
                let title = parts[0].to_string();
                let done = parts[1].parse::<bool>().unwrap_or(false);
                tasks.push(Task { title, done });
            }
        }
    }
    tasks
}
 
fn save(tasklist: &Vec<Task>){
    let mut file = File::create("tasks.txt").unwrap();

    for task in tasklist {
        let line = format!("{}|{}\n",task.title,task.done);
        let _ = file.write_all(line.as_bytes());
    }
}

//-----------------
fn show_menu(){
    println!("\n--- Task Manager Menu ---");
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
    println!("✅  Task Added!\n");
}

fn show_task(tasklist: &Vec<Task>){
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
            println!("Task Complete!👍");
        }    
    }
}

fn delete_task(tasklist:&mut Vec<Task>){
    let delete = get_input("[Delete] select: ");
    
    if let Ok(index) = delete.trim().parse::<usize>(){
        if index > 0 && index <= tasklist.len(){
            tasklist.remove(index-1);
            println!("✅  Task Deleted!");
        }
    }
}

fn get_input(show : &str) -> String{
    print!("{}",show);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
