extern crate colored;
use self::colored::*;
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub category: String,
}

impl Todo {
    fn new(title: String, category: String) -> Todo {
        Todo {
            title: title.yellow().to_string(),
            completed: false,
            category,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
        self.title = self.title.strikethrough().to_string();
    }
}

pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, title: String, category: String) {
        let new_todo = Todo::new(title, category);
        self.todos.push(new_todo);
    }

    pub fn mark_todo_completed(&mut self, index: usize) {
        self.todos[index].mark_completed();
    }
}
