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