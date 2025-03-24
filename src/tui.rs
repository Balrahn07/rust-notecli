use crate::note::load_notes;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};
use std::io::stdout;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("NoteCLI").borders(Borders::ALL);
            let notes = load_notes();
            let note_text = if notes.is_empty() {
                "No notes found.".to_string()
            } else {
                notes
                    .iter()
                    .map(|note| format!("[{}] {}", note.id, note.content))
                    .collect::<Vec<String>>()
                    .join("\n")
            };

            let paragraph = Paragraph::new(note_text).block(block);
            f.render_widget(paragraph, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
