mod create;
mod model;

use crate::create::create;

fn main() {
    let tarefa = create();
    println!("Tarefa criada: {} - {} - {}", tarefa.name, tarefa.date, tarefa.done);
}

