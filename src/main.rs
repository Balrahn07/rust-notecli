mod note;
mod tui;

use clap::{Parser, Subcommand};
use note::{load_notes, save_notes};

#[derive(Parser)]
#[command(name = "NoteCLI")]
#[command(about = "A simple CLI app to take notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { content: String },
    List,
    View { id: usize },
    Delete { id: usize },
    Search { keyword: String },
    Tui,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { content } => {
            let mut notes = load_notes();
            let note = note::add_note(content, &mut notes);
            save_notes(&notes).expect("Failed to save notes");
            println!("Note added: [{}] {}", note.id, note.content);
        }

        Commands::List => {
            let notes = load_notes();
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                for note in &notes {
                    println!("[{}] {}", note.id, note.content);
                }
            }
        }

        Commands::View { id } => {
            let notes = load_notes();
            match note::view_note(*id, &notes) {
                Some(n) => println!("[{}] {}", n.id, n.content),
                None => println!("Note with ID {} not found.", id),
            }
        }

        Commands::Delete { id } => {
            let mut notes = load_notes();
            if note::delete_note(*id, &mut notes) {
                save_notes(&notes).expect("Failed to save notes");
                println!("Note {} deleted ✅", id);
            } else {
                println!("Note with ID {} not found.", id);
            }
        }

        Commands::Search { keyword } => {
            let notes = load_notes();
            let results = note::search_notes(keyword, &notes);
            if results.is_empty() {
                println!("No notes matching '{}'", keyword);
            } else {
                for note in results {
                    println!("[{}] {}", note.id, note.content);
                }
            }
        }

        Commands::Tui => {
            if let Err(e) = tui::run_tui() {
                eprintln!("Error running TUI: {:?}", e);
            }
        }
    }
}
