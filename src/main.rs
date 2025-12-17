use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

fn get_tasks_file() -> PathBuf {
    let mut dir = dirs::home_dir().expect("Could not find home directory");

    dir.push(".task_manager");

    // Create ~/.task_manager if it doesn't exist
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Could not create ~/.task_manager directory");
    }

    dir.push("tasks.json");
    dir
}

fn load_tasks() -> Vec<Task> {
    let path = get_tasks_file();
    let data = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let path = get_tasks_file();
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(&path, json).expect("Unable to write file");
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

fn handle_command(command: &str, args: &[&str]) {
    match command {
        "add" => {
            if args.is_empty() {
                println!("Usage: add <task text>");
                return;
            }
        
            let text = args.join(" ");
            let text = text.trim_matches('"');
        
            add_task(text.to_string());
        }
        "list" => {
            list_tasks();
        }
        "done" => {
            if args.is_empty() {
                println!("Usage: done <id>");
                return;
            }
        
            let id: u32 = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid ID");
                    return;
                }
            };
        
            done(id);
        }
        "undone" => {
            if args.is_empty() {
                println!("Usage: undone <id>");
                return;
            }
        
            let id: u32 = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid ID");
                    return;
                }
            };
        
            undone(id);
        }
        "delete" => {
            if args.is_empty() {
                println!("Usage: delete <id>");
                return;
            }
        
            let id: u32 = match args[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid ID");
                    return;
                }
            };
        
            delete(id);
        }
        _ => {
            println!("Unknown command");
        }
    }
}

fn repl() {
    use std::io::{self, Write};

    println!("Task Manager REPL");
    println!("Type 'help' for commands.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Split into command + args
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        handle_command(command, &args);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: task_manager <command> [args]");
        return;
    }

    let command = &args[1];

    if args[1] == "repl" {
        repl();
        return;
    }

    match command.as_str() {
        "repl" => {
            repl()
        }
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
