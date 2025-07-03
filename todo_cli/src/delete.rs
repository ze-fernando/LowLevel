use crate::json_functions::delete_in_json;

pub fn delete_task_by_id(id: u32) -> bool{
    match delete_in_json(id) {
        Ok(_) => true,
        Err(_) => false,
    }
}

