use clap::{Parser, Subcommand};
use serde_json::Result;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "cli_todo")]
#[command(about = "A simple CLI To-Do list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Complete { index: usize },
    Remove { index: usize },
    Edit { index: usize, description: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
    description: String,
    done: bool,
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => {
            println!("Add: {}", description);
            let task = Task {
                description,
                done: false,
            };
            let mut tasks = load_tasks()?;
            tasks.push(task);
            let _ = save_tasks(&tasks);
        }
        Commands::List => {
            println!("List");
            let tasks = load_tasks()?;
            for (i, task) in tasks.iter().enumerate() {
                println!(
                    "{}: [{}] {}",
                    i,
                    if task.done { "x" } else { " " },
                    task.description
                );
            }
        }
        Commands::Complete { index } => {
            println!("Complete: {}", index);
            let mut tasks = load_tasks()?;
            if let Some(task) = tasks.get_mut(index) {
                task.done = true;

                let _ = save_tasks(&tasks);
            } else {
                println!("Invalid index");
            }
        }
        Commands::Remove { index } => {
            println!("Remove: {}", index);
            let mut tasks = load_tasks()?;
            if (index as usize) < tasks.len() {
                tasks.remove(index);
                let _ = save_tasks(&tasks);
            } else {
                println!("Invalid index");
            }
        }
        Commands::Edit { index, description } => {
            println!("Edit: {} {}", index, description);
            let mut tasks = load_tasks()?;
            if let Some(task) = tasks.get_mut(index) {
                task.description = description;
                let _ = save_tasks(&tasks);
            } else {
                println!("Invalid index");
            }
        }
    }

    Ok(())
}

fn load_tasks() -> Result<Vec<Task>> {
    let data = read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let data = serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
