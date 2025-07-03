use std::io::{self, Write};
use crate::{model::{Task}};
use crate::json_functions::update_in_json;

pub fn update(id: u32) -> bool {
    let mut name = String::new();
    let mut date = String::new();
    let mut done = String::new();

    print!("Nome: ");
    io::stdout().flush().unwrap();  // <--- força mostrar o texto antes do input
    io::stdin()
    .read_line(&mut name).expect("Digite um nome valido");


    print!("Prazo: ");
    io::stdout().flush().unwrap();  // <--- força mostrar o texto antes do input
    io::stdin()
    .read_line(&mut date).expect("Digite uma data valida");


    print!("Completo (sim/nao): ");
    io::stdout().flush().unwrap();  // <--- força mostrar o texto antes do input
    io::stdin()
    .read_line(&mut done).expect("Digite um valor valido");

    let done = match done.trim().to_lowercase().as_str() {
        "sim" => true,
        "nao" | "não" => false,
        _ => false,
    };

    let new_task = Task {
        id: id,
        name:name,
        date: date,
        done: done
    };

    match update_in_json(id, new_task) {
        Ok(_) => true,
        Err(_) => false,
    }
}

