use crate::db::get_tasks;

use std::io::{self, Write};


pub fn update(id: u32) -> bool {
    let mut arr = get_tasks();
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


    // procura a task pelo id mutável
    for task in arr.iter_mut() {
        if task.id == id {
            task.name = name;
            task.date = date;
            task.done = done;
            return true; 
        }
    }
    false 
}

