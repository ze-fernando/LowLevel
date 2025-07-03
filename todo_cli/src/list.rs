use crate::json_functions::get_json;

pub fn list_task(){
    let path = "./database.json";
    let array = get_json(path);
    for task in array {
        println!("\n{} - {} | {} | {}", task.id, task.name, task.date, task.done);
    }
}

pub fn list_by_id(id: u32){
    let path = "./database.json";
    let array = get_json(path);
    for task in array {
        if task.id == id {
            println!("\n{} - {} | {} | {}", task.id, task.name, task.date, task.done);
            break;
        }
    }
}
