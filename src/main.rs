mod model;

use model::todo::{State, Todo};

fn main() {
    let todo = Todo {
        id: 1,
        title: String::from("Learn Rust"),
        state: State::Backlog,
    };

    let serialized = serde_json::to_string(&todo).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Todo = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
