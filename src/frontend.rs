use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, enable_raw_mode, is_raw_mode_enabled},
};

use crate::editor::Editor;

pub fn run_frontend(editor: Editor) -> std::io::Result<()> {
    let mut stdout = stdout();

    // clear the screen and move to (0, 0)
    enable_raw_mode()?;

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;

    'running: loop {
        if let Event::Key(key) = read()? {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => break 'running,
                _ => (),
            }
        }
    }

    Ok(())
}
