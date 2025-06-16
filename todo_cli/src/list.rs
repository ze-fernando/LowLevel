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
