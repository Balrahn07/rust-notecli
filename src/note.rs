use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
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
    let mut file =
        OpenOptions::new().write(true).create(true).truncate(true).open(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note() {
        let content = String::from("Test note");
        let note = Note { id: 1, content: content.clone() };
        assert_eq!(note.id, 1);
        assert_eq!(note.content, content);
    }

    #[test]
    fn test_note_serialization() {
        let note = Note { id: 42, content: "Serialize me".into() };
        let json = serde_json::to_string(&note).unwrap();
        assert!(json.contains("Serialize me"));

        let deserialized: Note = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, 42);
        assert_eq!(deserialized.content, "Serialize me");
    }
}
