use crate::{db::get_tasks};


pub fn delete_task_by_id(id: u32) -> bool{
    let mut tasks = get_tasks();
    for (i, task) in tasks.iter().enumerate(){
        if task.id == id{
            tasks.remove(i);
            return true;
        }
    }
    return false;
}
