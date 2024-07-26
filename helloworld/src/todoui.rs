use std::io;
use todo::TodoList;

pub fn get_current_list(todolist: &TodoList) {
    let mut i = 1;
    for todo in &todolist.todos {
        println!("{}. {} : {}", i, todo.title, todo.category);
        i += 1;
    }
    println!(
        "Press 1 to add a new task, 2 to mark a task as completed, 3 to delete a task, 4 to exit"
    );
}

pub fn todo_ui() {
    let mut todo_list = TodoList::new();

    get_current_list(&todo_list);

    while true {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                println!("Enter the title of the task: ");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");

                println!("Enter the category of the task: ");
                let mut category = String::new();
                io::stdin()
                    .read_line(&mut category)
                    .expect("Failed to read line");

                todo_list.add_todo(title.trim().to_string(), category.trim().to_string());
                get_current_list(&todo_list);
            }
            2 => {
                println!("Enter the index of the task you want to mark as completed: ");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = index.trim().parse().expect("Please type a number!");

                todo_list.mark_todo_completed(index - 1);
                get_current_list(&todo_list);
            }
            3 => {
                println!("Enter the index of the task you want to delete: ");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = index.trim().parse().expect("Please type a number!");

                todo_list.todos.remove(index - 1);
                get_current_list(&todo_list);
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
                get_current_list(&todo_list);
            }
        }
    }
}
