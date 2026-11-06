mod model;

use model::todo::{State, Todo};

fn add_todo(todo_list: &mut Vec<Todo>, title: String, state: State) {
    let id = todo_list.len() as u32 + 1;
    let todo = Todo {
        id,
        title: title.trim().to_string(),
        state,
    };
    todo_list.push(todo);
}

fn main() {
    println!("Welcome to the Todo App!");

    let mut todo_list: Vec<Todo> = Vec::new();
    let mut title = String::new();
    let mut state_input = String::new();

    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    std::io::stdin()
        .read_line(&mut state_input)
        .expect("Failed to read line");

    let state = match state_input.trim() {
        "Backlog" => State::Backlog,
        "InProgress" => State::InProgress,
        "Completed" => State::Completed,
        _ => {
            println!("Invalid state input. Defaulting to Backlog.");
            State::Backlog
        }
    };

    add_todo(&mut todo_list, title, state);

    let serialized = serde_json::to_string(&todo_list).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Vec<Todo> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
