mod model;
mod create;
mod update;
mod delete;
mod list;
mod json_functions;

use std::io::{self, Write};
use create::create;
use update::update;
use delete::delete_task_by_id;
use list::{list_task, list_by_id};

fn main() {

    loop {
        println!("\n==== MENU ====");
        println!("1 - Criar tarefa");
        println!("2 - Listar todas");
        println!("3 - Buscar por ID");
        println!("4 - Atualizar tarefa");
        println!("5 - Deletar tarefa");
        println!("0 - Sair");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        print!("\n");
        match option.trim() {
            "1" => {
                let _ = create();
                println!("Tarefa criada!");
            }
            "2" => {
                list_task();
                println!("Tarefas listadas!");
            }
            "3" => {
               let id = input_id();
                list_by_id(id);
            }
            "4" => {
                let id = input_id();
                if update(id) {
                    println!("Tarefa atualizada!");
                } else {
                    println!("Tarefa com ID {} não encontrada.", id);
                }
            }
            "5" => {
                let id = input_id();
                if delete_task_by_id(id) {
                    println!("Tarefa removida.");
                } else {
                    println!("ID inválido.");
                }
            }
            "0" => break,
            _ => println!("Opção inválida."),
        }
    }
}

fn input_id() -> u32 {
    print!("Digite o ID: ");
    io::stdout().flush().unwrap();
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();
    id_str.trim().parse().unwrap_or(0)
}

