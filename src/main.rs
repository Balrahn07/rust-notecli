use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { content } => {
            println!("Creating a new note: {}", content);
        }
        Commands::List => {
            println!("Listing all notes...");
        }
        Commands::View { id } => {
            println!("Viewing note with ID: {}", id);
        }
        Commands::Delete { id } => {
            println!("Deleting note with ID: {}", id);
        }
        Commands::Search { keyword } => {
            println!("Searching notes for keyword: {}", keyword);
        }
    }
}
