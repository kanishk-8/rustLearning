use std::io;

struct TodoApp {
    tasks: Vec<Task>,
}

struct Task {
    title: String,
    completed: bool,
}

impl Task {
    fn new(title: String) -> Task {
        Task {
            title,
            completed: false,
        }
    }
}

fn main() {
    println!("Welcome to the todo app");
    let mut todoapp = TodoApp { tasks: vec![] };

    loop {
        println!("\nSelect an action: 1. add todo 2. remove todo 3. view todos 4. exit app");

        let mut actions = String::new();
        io::stdin()
            .read_line(&mut actions)
            .expect("Failed to read the line");

        let user_action: i32 = match actions.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match user_action {
            1 => {
                println!("You have selected to add a todo");
                add_todo(&mut todoapp);
            }
            2 => {
                println!("You have selected to remove a todo");
                remove_todo(&mut todoapp);
            }
            3 => {
                println!("You have selected to view todos");
                view_todos(&todoapp);
            }
            4 => {
                println!("Exiting the app");
                break;
            }
            _ => println!("Invalid action selected"),
        }
    }
}

fn add_todo(todoapp: &mut TodoApp) {
    println!("Enter the todo to add:");
    let mut task_title = String::new();
    io::stdin()
        .read_line(&mut task_title)
        .expect("Failed to read task");

    let trimmed_title = task_title.trim().to_string();
    if trimmed_title.is_empty() {
        println!("Task cannot be empty!");
        return;
    }

    let task = Task::new(trimmed_title);
    todoapp.tasks.push(task);
    println!("Todo added successfully!");
}

fn remove_todo(todoapp: &mut TodoApp) {
    if todoapp.tasks.is_empty() {
        println!("No todos to remove!");
        return;
    }

    view_todos(&todoapp);
    println!(
        "Enter the number of the todo to remove (1-{}):",
        todoapp.tasks.len()
    );

    let mut index_input = String::new();
    io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read the line");

    let user_index: usize = match index_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if user_index >= 1 && user_index <= todoapp.tasks.len() {
        let actual_index = user_index - 1;
        todoapp.tasks.remove(actual_index);
        println!("Todo removed successfully!");
    } else {
        println!(
            "Invalid index. Please enter a number between 1 and {}",
            todoapp.tasks.len()
        );
    }
}

fn view_todos(todoapp: &TodoApp) {
    if todoapp.tasks.is_empty() {
        println!("No todos found!");
        return;
    }

    println!("Your todos:");
    for (i, task) in todoapp.tasks.iter().enumerate() {
        let status = if task.completed {
            "completed"
        } else {
            "pending"
        };
        println!("{}. {} [{}]", i + 1, task.title, status);
    }
}
