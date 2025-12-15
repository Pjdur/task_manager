use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", json).expect("Unable to write file");
}

fn add_task(text: String) {
    let mut tasks = load_tasks();

    let id = (tasks.len() as u32) + 1;

    tasks.push(Task {
        id,
        text,
        done: false,
    });

    save_tasks(&tasks);

    println!("Added task #{id}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: task_manager <command> [args]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: task_manager add <task text>");
                return;
            }

            let text = args[2..].join(" ");
            add_task(text);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
