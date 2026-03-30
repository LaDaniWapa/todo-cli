mod db;
mod errors;

use crate::db::{Result, TodoDb};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { title: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: i64 },
    /// Mark a task as not done
    Undone { id: i64 },
    /// Delete a todo
    Delete { id: i64 },
    /// Clears all done tasks
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let con = TodoDb::new()?;
    con.init()?;

    match cli.command {
        Commands::Add { title } => con.add_task(&title),
        Commands::List => {
            let tasks = con.list_tasks()?;

            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                tasks.iter().for_each(|t| println!("{t}"))
            }

            Ok(())
        }
        Commands::Done { id } => con.done_task(id),
        Commands::Undone { id } => con.undone_task(id),
        Commands::Delete { id } => con.delete_task(id),
        Commands::Clear => {
            let cleared = con.clear_done_tasks()?;
            Ok(println!("Removed {cleared} tasks"))
        }
    }
}
