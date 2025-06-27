use std::i32;
use std::io;

struct Args {
    task: String,
    priority: i32,
    due_date: String,
}

fn main() {
    println!("Enter a to do list item:");
    let mut enter_task = 1;
    while enter_task == 1 {
        let mut task = Args {
            task: String::new(),
            priority: 0,
            due_date: "".to_string()
        };
        let mut input = String::new();
        println!("Enter Task Name:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to Read Line");
        task.task = input.trim().to_string();
        input.clear();
        println!("Enter Priority (1-5):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to Read Line");
        task.priority = input.trim().parse().expect("Enter a valid number");
        input.clear();
        println!("Enter Due Date (dd/mm/yyyy):");
    }

}
