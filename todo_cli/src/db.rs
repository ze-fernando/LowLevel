use crate::model::Task;

pub fn get_tasks() -> Vec<Task> {
    vec![
    Task {
            id: 0,
            name: "Reunião com equipe".to_string(),
            date: "2025-06-21".to_string(),
            done: false,
        },
    ]
}
