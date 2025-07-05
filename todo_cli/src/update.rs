
use std::io::{self, Write};
use crate::{model::Task};
use crate::json_functions::update_in_json;

pub fn update(id: u32) -> bool {
    let mut name = String::new();
    let mut date = String::new();
    let mut done = String::new();

    print!("Nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Digite um nome válido");
    let name = name.trim().to_string();

    print!("Prazo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date).expect("Digite uma data válida");
    let date = date.trim().to_string();

    print!("Completo (sim/nao): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut done).expect("Digite um valor válido");

    let done = matches!(done.trim().to_lowercase().as_str(), "sim");

    let new_task = Task {
        id,
        name,
        date,
        done,
    };

    update_in_json(id, new_task).is_ok()
}

