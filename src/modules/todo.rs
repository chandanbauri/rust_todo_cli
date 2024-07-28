pub struct TodoApp {
    pub todo_list: Vec<Todo>,
}

impl TodoApp {
    pub fn initialise() -> Self {
        TodoApp {
            todo_list: Vec::new(),
        }
    }
    pub fn display_all(&self) {
        for td in self.todo_list.iter() {
            td.display_todo();
        }
    }

    pub fn mark_complete(&mut self, index: usize) {
        if index < self.todo_list.len() {
            self.todo_list[index].mark_complete();
        } else {
            println!("Index out of bounds");
        }
    }

    pub fn create_todo (&mut self,title: &str){
        let todo = Todo::new(title); 
        self.todo_list.push(todo);
    }
}

pub struct Todo {
    pub title: String,
    pub completed: bool,
}

impl Todo {
    fn new(title: &str) -> Self {
        Todo {
            title: title.to_string(),
            completed: false,
        }
    }
    fn display_todo(&self) {
        println!("--{}--",self.title);
        println!("Completed: {}", self.completed);
        println!("-----------------------");
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }
}
