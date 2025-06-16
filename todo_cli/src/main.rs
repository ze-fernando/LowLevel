mod create;
mod model;
mod db;
mod list;

use crate::{db::get_tasks, list::{list_by_id, list_task}};


fn main() {
    let tasks = get_tasks();

    list_by_id(2, tasks);
}

