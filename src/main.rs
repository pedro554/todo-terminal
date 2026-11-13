mod model;

use model::todo::{State, Todo};
use std::fs::File;
use std::io::BufWriter;

fn add_todo(todo_list: &mut Vec<Todo>, title: String, state: State) {
    let id = todo_list.len() as u32 + 1;
    let todo = Todo {
        id,
        title: title.trim().to_string(),
        state,
    };
    todo_list.push(todo);
}

fn gravar(todo_list: &mut Vec<Todo>){
    let filename = "todo.json";
    let file = match File::create(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("Erro ao criar arquivo {}", error);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    if let Err(error) = serde_json::to_writer_pretty(&mut writer, &todo_list) {
        println!("Erro ao criar arquivo {}", error);
    }
}

fn carregar_json() -> String {
    let filename = "todo.json";
    let data = std::fs::read_to_string(filename).unwrap();
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON inválido.");
    println!("{}", json);

    json.to_string()
}

fn main() -> std::io::Result<()> {
    println!("Welcome to the Todo App!");

    let dados_carregados = carregar_json();

    let mut todo_list: Vec<Todo> = serde_json::from_str(&dados_carregados)?;

    println!("{:?}", todo_list);

    loop {
        let mut title = String::new();
        let mut state_input = String::new();

        println!("Informe o titulo:");
        std::io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");

        println!("informe o estado Backlog, InProgress ou Completed");
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

        gravar(&mut todo_list);
    }
}
