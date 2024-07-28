mod modules;
use modules::todo;
use std::io;

fn main() {
    let mut todo_app = todo::TodoApp::initialise();

    fn show_menu() {
        println!("---------------------------------------");
        println!("create todo - 1");
        println!("mark todo as complete - 2");
        println!("display all todo - 3");
        println!("---------------------------------------");

    }

    fn create_todo(todo_app: &mut todo::TodoApp) {
        println!("---------------------------------------");
        println!("Enter todo title: ");
        let mut todo_title = String::new();

        let result = io::stdin().read_line(&mut todo_title);

        match result {
            Ok(_) => {
                todo_app.create_todo(&mut todo_title);
            }
            Err(e) => println!("Error reading input {}", e),
        }
        println!("---------------------------------------");

    }

    fn mark_todo_complete(todo_app: &mut todo::TodoApp) {
        println!("---------------------------------------");
        println!("Enter todo index: ");
        let mut option_selected = String::new();

        let result = io::stdin().read_line(&mut option_selected);

        let option_selected = option_selected.trim();

        match result {
            Ok(_) => {
                let parsed_option: Result<usize, _> = option_selected.parse();

                match parsed_option {
                    Ok(n) => todo_app.mark_complete(n),
                    Err(e) => println!("Failed to parse number {}", e),
                }
            }
            Err(e) => println!("Error reading input {}", e),
        }
        println!("---------------------------------------");
    }

    fn trigger_action(option_selected: &String, todo_app: &mut todo::TodoApp) {
        match option_selected.trim() {
            "1" => create_todo(todo_app),
            "2" => mark_todo_complete(todo_app),
            "3" => todo_app.display_all(),
            _ => show_menu(),
        }
    }

    loop {
        if !(true) {
            break;
        }

        show_menu();
        let mut option_selected = String::new();

        let result = io::stdin().read_line(&mut option_selected);

        match result {
            Ok(_) => trigger_action(&option_selected, &mut todo_app),
            Err(e) => println!("Failed to read input {}", e),
        }
    }
}
