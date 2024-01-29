use lazy_static::lazy_static;

struct Task {
    id: u32,
    description: String,
    is_completed: bool,
}

static mut PUB_ID: u32 = 1;

lazy_static! {
    static ref TASKS: Vec<Task> = Vec::new();
}

fn main() {
    loop {
        println!("Pick what to do");
        println!("1. Add Task");
        println!("2. Complete Task");
        println!("3. List Tasks");
        println!("4. Exit");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let choice: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter description for your Task:");
                let mut task_description = String::new();
                std::io::stdin().read_line(&mut task_description).unwrap();

                let new_task = add_task(task_description.trim());
                println!("Task added with ID: {}", new_task.id);
            }
            2 => {
                println!("Enter the ID of the task to complete:");
                let mut task_id_str = String::new();
                std::io::stdin().read_line(&mut task_id_str).unwrap();

                let task_id: usize = match task_id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if let Some(completed_task) = complete_task(task_id) {
                    println!("Task completed: ID={}, Description={}", completed_task.id, completed_task.description);
                } else {
                    println!("Task not found.");
                }
            }
            3 => list_tasks(),
            4 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }
}

fn add_task(description: &str) -> Task {
    let new_task = Task {
        id: unsafe {
            let current_id = PUB_ID;
            PUB_ID += 1;
            current_id
        },
        description: description.to_string(),
        is_completed: false,
    };

    unsafe {
        TASKS.push(new_task.clone());
    }
    new_task
}

fn complete_task(id: usize) -> Option<&Task> {
    unsafe {
        if let Some(task) = TASKS.get_mut(id - 1) {
            task.is_completed = true;
            Some(task)
        } else {
            None
        }
    }
}

fn list_tasks() {
    for task in &*TASKS {
        println!(
            "ID: {}, Description: {}, Completed: {}",
            task.id, task.description, task.is_completed
        );
    }
}
