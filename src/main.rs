mod model;

use model::todo::{State, Todo};
fn main() {
    println!("Welcome to the Todo App!");

    let mut title = String::new();
    let mut state_input = String::new();

    std::io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    std::io::stdin()
        .read_line(&mut state_input)
        .expect("Failed to read line");

    let todo = Todo {
        id: 1,
        title: title.trim().to_string(),
        state: match state_input.trim() {
            "Backlog" => State::Backlog,
            "InProgress" => State::InProgress,
            "Completed" => State::Completed,
            _ => {
                println!("Invalid state input. Defaulting to Backlog.");
                State::Backlog
            }
        },
    };

    let serialized = serde_json::to_string(&todo).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Todo = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
