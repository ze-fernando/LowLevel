use crate::{db::get_tasks};

pub fn list_task(){
    let array = get_tasks();
    for task in array {
        println!("{} {} {}", task.name, task.date, task.done);
    }
}

pub fn list_by_id(id: u32){
    let array = get_tasks();
    for task in array {
        if task.id == id {
            println!("{} {} {}", task.name, task.date, task.done);
            break;
        }
    }
}
