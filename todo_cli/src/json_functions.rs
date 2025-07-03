use crate::model::{Task, TaskList};
use std::io::{Write};
use std::fs::{File};
use serde_json::{self, Result};


pub fn save_in_json(task: &Task) -> Result<()> {
    let path = "./database.json";
    let mut all_tasks = get_json(path);

    all_tasks.push(task.clone());

    let json = serde_json::to_string_pretty(&all_tasks)?;
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
    Ok(())
}


pub fn update_in_json(id: u32, task: Task) -> Result<()> {
    delete_in_json(id)?;
    save_in_json(&task)?;
    Ok(())
}

pub fn delete_in_json(id: u32) -> Result<()> {
    let path = "./database.json";
    let mut all_tasks = get_json(path);

    all_tasks.retain(|task| task.id != id);
    let json = serde_json::to_string_pretty(&all_tasks)?;
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
    Ok(())
}

pub fn get_json(path: &str) -> TaskList{
    let tasks: Vec<Task> = match File::open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
        }
        Err(_) => vec![], // se não existir, começa com lista vazia
    };

    tasks
}
