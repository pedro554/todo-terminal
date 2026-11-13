mod model;

use model::todo::{State, Todo};
use std::fs::File;
use std::io::{BufWriter, Write};

fn add_todo(todo_list: &mut Vec<Todo>, title: String, state: State) {
    if title.trim().is_empty() {
        println!("O título não pode estar vazio!");
        return;
    }

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

    writer.flush().expect("Falha ao escrever no arquivo json");
}

fn carregar_json() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let filename = "todo.json";

    if !std::path::Path::new(filename).exists() {
        return Ok(Vec::new());
    }

    let data = std::fs::read_to_string(filename)?;

    let lista: Vec<Todo> = serde_json::from_str(&data)?;

    Ok(lista)
}

fn ler_texto(mensagem: &str) -> std::io::Result<String> {
    println!("{}", mensagem);

    let mut texto = String::new();

    std::io::stdin().read_line(&mut texto)?;

    Ok(texto.trim().to_string())
}

fn main() -> std::io::Result<()> {
    let mut todo_list: Vec<Todo> = carregar_json().expect("Falha ao carregar json");

    loop {
        let title = ler_texto("Informe o título")?;
        let state_input = ler_texto("informe o estado Backlog, InProgress ou Completed")?;

        let state = match state_input.trim() {
            "Backlog" => State::Backlog,
            "InProgress" => State::InProgress,
            "Completed" => State::Completed,
            _ => State::Backlog
        };

        add_todo(&mut todo_list, title, state);

        gravar(&mut todo_list);
    }
}
