use crate::model::Task;

use std::io::{self, Write};


pub fn create() -> Task{
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

    Task {
        id: 5,
        name: name.trim().to_string(),
        date: date.trim().to_string(),
        done: done
    }
}
