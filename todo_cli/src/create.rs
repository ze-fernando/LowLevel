use crate::{model::{Task}};
use rand::Rng;
use std::io::{self, Write};
use std::fs::{File};
use serde_json::{self, Error};


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


fn save_in_json(task: &Task) -> Result<(), Error> {
    let path = "../database.json";

    let tasks: Vec<Task> = match File::open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
        }
        Err(_) => vec![], // se não existir, começa com lista vazia
    };

    let mut all_tasks = tasks;
    all_tasks.push(task.clone());

    let json = serde_json::to_string_pretty(&all_tasks)?;
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
    Ok(())
}

