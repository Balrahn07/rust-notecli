use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_PATH: &str = "notes.json";


#[derive(Parser)]
#[command(name = "NoteCLI")]
#[command(about = "A simple CLI app to take notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    New {
        /// The content of the note
        content: String,
    },

    /// List all notes
    List,

    /// View a specific note by ID
    View {
        /// The ID of the note
        id: usize,
    },

    /// Delete a note by ID
    Delete {
        /// The ID of the note
        id: usize,
    },

    /// Search notes by keyword
    Search {
        /// The keyword to search for
        keyword: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: usize,
    content: String,
}

// Read notes from file (or return empty vec if not exists or error)
fn load_notes() -> Vec<Note> {
    match fs::read_to_string(FILE_PATH) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

// Write notes back to the file
fn save_notes(notes: &Vec<Note>) -> io::Result<()> {
    let data = serde_json::to_string_pretty(notes)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { content } => {
            let mut notes = load_notes();
            let id = notes.last().map_or(1, |n| n.id + 1);
            let note = Note {
                id,
                content: content.to_string(),
            };
            notes.push(note);
            save_notes(&notes).expect("Failed to save notes");
            println!("Note added ✅");
        }

        Commands::List => {
            let notes = load_notes();
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                for note in notes {
                    println!("[{}] {}", note.id, note.content);
                }
            }
        }

        Commands::View { id } => {
            let notes = load_notes();
            match notes.iter().find(|n| n.id == *id) {
                Some(note) => println!("[{}] {}", note.id, note.content),
                None => println!("Note with ID {} not found.", id),
            }
        }

        Commands::Delete { id } => {
            let mut notes = load_notes();
            let original_len = notes.len();
            notes.retain(|n| n.id != *id);
            if notes.len() < original_len {
                save_notes(&notes).expect("Failed to save notes");
                println!("Note {} deleted ✅", id);
            } else {
                println!("Note with ID {} not found.", id);
            }
        }

        Commands::Search { keyword } => {
            let notes = load_notes();
            let results: Vec<&Note> = notes.iter()
                .filter(|n| n.content.to_lowercase().contains(&keyword.to_lowercase()))
                .collect();

            if results.is_empty() {
                println!("No notes matching '{}'", keyword);
            } else {
                for note in results {
                    println!("[{}] {}", note.id, note.content);
                }
            }
        }
    }
}