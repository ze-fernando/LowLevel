use crate::model::TaskList;

pub fn list_task(array: TaskList){
    for(i, task) in array.iter().enumerate(){
        println!(
        "[{}] - {} {} {}",
        i +1,
        task.name,
        task.date,
        task.done);
    }
}

pub fn list_by_id(id: u32, array: TaskList){
    for(i, task) in array.iter().enumerate(){
           if(task.id == id){
             println!(
                "[{}] - {} {} {}",
                i +1,
                task.name,
                task.date,
                task.done
             );
                break;
        }
    }
}
