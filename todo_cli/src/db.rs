use crate::model::Task;

pub fn get_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 0,
            name: "Estudar Rust".to_string(),
            date: "2025-06-17".to_string(),
            done: false,
        },
        Task {
            id: 1,
            name: "Ler livro".to_string(),
            date: "2025-06-18".to_string(),
            done: true,
        },
        Task {
            id: 2,
            name: "Fazer exercícios".to_string(),
            date: "2025-06-19".to_string(),
            done: false,
        },
        Task {
            id: 3,
            name: "Enviar relatório".to_string(),
            date: "2025-06-20".to_string(),
            done: true,
        },
        Task {
            id: 4,
            name: "Reunião com equipe".to_string(),
            date: "2025-06-21".to_string(),
            done: false,
        },
    ]
}
