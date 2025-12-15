use serde::{Deserialize, Serialize};
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

fn list_tasks() {
    let tasks = load_tasks();

    if tasks.is_empty() {
        println!("No tasks yet.");
        return;
    }

    for task in tasks {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{} {}: {}", status, task.id, task.text);
    }
}

fn done(id: u32) {
    let mut tasks = load_tasks();

    for task in tasks.iter_mut() {
        if task.id == id {
            task.done = true;
        }
    }

    save_tasks(&tasks);
    println!("Marked task #{id} as done");
}

fn undone(id: u32) {
    let mut tasks = load_tasks();

    for task in tasks.iter_mut() {
        if task.id == id {
            task.done = false;
        }
    }

    save_tasks(&tasks);
    println!("Marked task #{id} as not done");
}

fn delete(id: u32) {
    let mut tasks = load_tasks();

    let original_len = tasks.len();

    // Keep only the tasks whose id is NOT the one we want to delete
    tasks.retain(|task| task.id != id);

    if tasks.len() == original_len {
        println!("No task with id {id} found");
        return;
    }

    save_tasks(&tasks);
    println!("Deleted task #{id}");
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
        "list" => {
            list_tasks();
        }
        "done" => {
            if args.len() < 3 {
                println!("Usage: task_manager done <id>");
                return;
            }

            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                println!("Invalid ID");
                std::process::exit(1);
            });

            done(id);
        }
        "undone" => {
            if args.len() < 3 {
                println!("Usage: task_manager undone <id>");
                return;
            }

            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                println!("Invalid ID");
                std::process::exit(1);
            });

            undone(id);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: task_manager delete <id>");
                return;
            }

            let id: u32 = args[2].parse().unwrap_or_else(|_| {
                println!("Invalid ID");
                std::process::exit(1);
            });

            delete(id);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
