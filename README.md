Simple Todo List CLI 📝
Welcome to my first Rust program! This is a command-line Todo List manager.
🎯 Features

Add new tasks with auto-generated IDs
Remove tasks by ID
View all current tasks
Simple command-line interface
Memory-safe operations (though using unsafe blocks for learning purposes)

🚀 Getting Started
Prerequisites
Make sure you have Rust installed on your system. If you don't have it yet:

Install Rust by following the official guide at https://www.rust-lang.org/tools/install
Verify your installation by running:
bashCopyrustc --version
cargo --version


Running the Program

Clone or download this repository
Navigate to the project directory
Build and run the program:
bashCopycargo run


📖 How to Use
The program presents a simple menu with 4 options:

Add a task (Option 1)

Enter the task description when prompted
The program automatically assigns an ID


Remove a task (Option 2)

Enter the ID of the task you want to remove
The task will be permanently deleted


View all tasks (Option 3)

Displays all current tasks with their IDs
Shows a message if the list is empty


Exit (Option 4)

Closes the program



🔍 Code Structure

Uses a static mutable vector to store tasks
Each task is stored as a tuple of (usize, &'static str)
Main functions:

create_list(): Adds new tasks
remove_item(): Removes tasks by ID
display_list(): Shows all tasks
