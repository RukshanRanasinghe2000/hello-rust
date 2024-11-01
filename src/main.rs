use std::io;

static mut TO_DO_LIST: Vec<(usize, &'static str)> = Vec::new();


// create TODO list

fn create_list(id: usize, work: &'static str) {
    unsafe {
        TO_DO_LIST.push((id, work));
    }
}

// Remove item form the TODO list

fn remove_item(n: usize) {
    unsafe {
        TO_DO_LIST.retain(|&(id, _)| id != n);
    }
}


// Display existing TODO list

fn display_list() {
    unsafe {
        if TO_DO_LIST.is_empty() {
            println!("The to-do list is empty.");
        } else {
            println!("To-Do List:");
            for (id, work) in &TO_DO_LIST {
                println!("ID: {}, Task: {}", id, work);
            }
        }
    }
}

fn main() {
    loop {
        println!("\nTo-Do List Manager");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. View all tasks");
        println!("4. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut task = String::new();
                println!("Enter the task description: ");
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");
                let task = task.trim();

                unsafe {
                    let id = TO_DO_LIST.len() + 1;
                    create_list(id, Box::leak(task.to_string().into_boxed_str()));
                }
                println!("Task added successfully!");
            }
            2 => {
                let mut id = String::new();
                println!("Enter the task ID to remove: ");
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line");
                let id: usize = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };

                remove_item(id);
                println!("Task removed successfully!");
            }
            3 => {
                display_list();
            }
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

