use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: usize,
    pub content: String,
}

const FILE_PATH: &str = "notes.json";

pub fn load_notes() -> Vec<Note> {
    match fs::read_to_string(FILE_PATH) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

pub fn save_notes(notes: &Vec<Note>) -> io::Result<()> {
    let data = serde_json::to_string_pretty(notes)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add_note(content: &str, notes: &mut Vec<Note>) -> Note {
    let id = notes.last().map_or(1, |n| n.id + 1);
    let note = Note {
        id,
        content: content.to_string(),
    };
    notes.push(note.clone());
    note
}

pub fn delete_note(id: usize, notes: &mut Vec<Note>) -> bool {
    let original_len = notes.len();
    notes.retain(|n| n.id != id);
    notes.len() < original_len
}

pub fn view_note(id: usize, notes: &[Note]) -> Option<&Note> {
    notes.iter().find(|n| n.id == id)
}

pub fn search_notes<'a>(keyword: &str, notes: &'a [Note]) -> Vec<&'a Note> {
    notes
        .iter()
        .filter(|n| n.content.to_lowercase().contains(&keyword.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note() {
        let content = String::from("Test note");
        let note = Note {
            id: 1,
            content: content.clone(),
        };
        assert_eq!(note.id, 1);
        assert_eq!(note.content, content);
    }

    #[test]
    fn test_note_serialization() {
        let note = Note {
            id: 42,
            content: "Serialize me".into(),
        };
        let json = serde_json::to_string(&note).unwrap();
        assert!(json.contains("Serialize me"));

        let deserialized: Note = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, 42);
        assert_eq!(deserialized.content, "Serialize me");
    }

    #[test]
    fn test_add_note_increments_id() {
        let mut notes = vec![];
        let note1 = add_note("First note", &mut notes);
        let note2 = add_note("Second note", &mut notes);

        assert_eq!(note1.id, 1);
        assert_eq!(note2.id, 2);
        assert_eq!(notes.len(), 2);
    }

    #[test]
    fn test_delete_note_removes_by_id() {
        let mut notes = vec![
            Note {
                id: 1,
                content: "A".into(),
            },
            Note {
                id: 2,
                content: "B".into(),
            },
        ];
        let deleted = delete_note(1, &mut notes);

        assert!(deleted);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, 2);
    }

    #[test]
    fn test_search_notes_matches_keyword() {
        let notes = vec![
            Note {
                id: 1,
                content: "Learn Rust".into(),
            },
            Note {
                id: 2,
                content: "Buy milk".into(),
            },
        ];
        let result = search_notes("rust", &notes);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "Learn Rust");
    }

    #[test]
    fn test_view_note_returns_correct_note() {
        let notes = vec![
            Note {
                id: 1,
                content: "Hello".into(),
            },
            Note {
                id: 2,
                content: "World".into(),
            },
        ];
        let note = view_note(2, &notes);

        assert!(note.is_some());
        assert_eq!(note.unwrap().content, "World");
    }
}
