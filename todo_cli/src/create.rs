use crate::{model::{Task}};
use rand::Rng;
use std::io::{self, Write};
use serde_json::{self, Error};
use crate::json_functions::save_in_json;

fn generate_new_id() -> u32{
    let mut rng = rand::thread_rng();
    rng.gen_range(10000..=99999)
}


pub fn create() -> Result<Task, Error> {
    let mut name = String::new();
    let mut date = String::new();
    let mut done = String::new();

    print!("Nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Erro no nome");

    print!("Prazo: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date).expect("Erro na data");

    print!("Completo (sim/nao): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut done).expect("Erro no status");

    let done = matches!(done.trim().to_lowercase().as_str(), "sim");

    let task = Task {
        id: generate_new_id(),
        name: name.trim().to_string(),
        date: date.trim().to_string(),
        done,
    };

    save_in_json(&task)?;

    Ok(task)
}

