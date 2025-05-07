use clap::Parser;
use rust_todo::Todo;
use std::fs::{self, File};
use std::io::{self, Write};

/// Simple program to make a todo list
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug, Clone)]
enum Command {
    /// Add a new todo
    Add {
        /// The todo item to add
        item: Vec<String>,
    },
    /// List all todos
    List,
    /// Marks a todo as done
    Done {
        /// The index of the todo to remove
        index: Vec<usize>,
    },
    /// Makrs a todo as not done
    Undo {
        /// The index of the todo to remove
        index: Vec<usize>,
    },
    /// erase all done todos
    Erase,
    /// clear all todos
    Clear,
}

fn save_todos(todos: &Vec<Todo>, file_path: &str) -> io::Result<()> {
    let json = serde_json::to_string(todos)?; // Serialize the todos to JSON
    let mut file = File::create(file_path)?; // Create or overwrite the file
    file.write_all(json.as_bytes())?; // Write the JSON data to the file
    Ok(())
}

fn load_todos(file_path: &str) -> io::Result<Vec<Todo>> {
    let data = fs::read_to_string(file_path)?; // Read the file as a string
    let todos: Vec<Todo> = serde_json::from_str(&data)?; // Deserialize the JSON data into a Vec<Todo>
    Ok(todos)
}

fn main() {
    let args = Args::parse();
    let file_path = "todos.json";

    let mut todos = load_todos(file_path).unwrap_or_else(|_| Vec::new());

    match args.command {
        Command::Add { item } => {
            let mut display_todo = String::new();
            for i in item.iter() {
                let id = todos.len() + 1;
                let todo = Todo::new(i.clone(), id);
                display_todo = if display_todo.is_empty() { todo.to_string() } else {format!("{}\n{}", display_todo, todo.to_string())};
                todos.push(todo);
            }
            save_todos(&todos, file_path).expect("Failed to save todos");
            println!("Added todos: {}", display_todo);
        }
        Command::List => {
            let display_list = todos.iter().map(|todo| todo.to_string()).collect::<Vec<String>>().join("\n");
            println!("{}", display_list);
        }
        Command::Done { index } => {
            let mut display_todo = String::new();
            if index.is_empty() {
                println!("No index provided");
                return;
            }
            for i in index.iter() {
                let ind = *i - 1;
                if ind >= todos.len() {
                    println!("Invalid index");
                    return;
                }
                todos[ind].mark_done();
                display_todo = format!("{}\n{}", display_todo, todos[ind].to_string());
            }
            
            save_todos(&todos, file_path).expect("Failed to save todos");
            println!("Marked following todos as done: {}", display_todo);
        }
        Command::Undo { index } => {
            let mut display_todo = String::new();
            if index.is_empty() {
                println!("No index provided");
                return;
            }
            for i in index.iter() {
                let ind = *i - 1;
                if ind >= todos.len() {
                    println!("Invalid index");
                    return;
                }
                todos[ind].mark_undone();
                display_todo = format!("{}\n{}", display_todo, todos[ind].to_string());
            }

            save_todos(&todos, file_path).expect("Failed to save todos");
            println!("Marked following todos as undone: {}", display_todo);
        }
        Command::Erase => {
            todos.retain(|todo| !todo.done); // Remove all done todos
            // Update IDs after removing done todos
            for (i, todo) in todos.iter_mut().enumerate() {
                todo.id = i;
            }
            save_todos(&todos, file_path).expect("Failed to save todos");
            println!("Erased all done todos");
        }
        Command::Clear => {
            todos.clear(); // Clear all todos
            save_todos(&todos, file_path).expect("Failed to save todos");
            println!("Cleared all todos");
        }
    }
}