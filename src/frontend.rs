use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use crate::editor::Editor;

pub struct Frontend {
    editor: Editor,
    stdout: Stdout,
    running: bool,
}

impl Frontend {
    pub fn new(editor: Editor) -> Self {
        Self {
            editor,
            running: true,
            stdout: stdout(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        enable_raw_mode().map_err(|e| e.to_string())?;

        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )
        .map_err(|e| e.to_string())?;

        while self.running {
            self.draw_text()?;
            self.handle_input().map_err(|e| e.to_string())?;
        }

        clean_up();

        Ok(())
    }

    fn handle_input(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = read()? {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => self.running = false,
                _ => (),
            }
        }

        Ok(())
    }

    fn draw_text(&mut self) -> Result<(), String> {
        // TODO: hide cursor

        let terminal_size = terminal::size().map_err(|e| e.to_string())?;
        let text = self.editor.get_text(terminal_size.0)?;

        let cursor_pos = self.editor.get_cursor_pos();
        let mut offset = cursor_pos.0;

        for row in text {
            offset += 1;

            self.stdout
                .write_all(row.as_bytes())
                .map_err(|e| e.to_string())?;

            execute!(self.stdout, MoveTo(0, offset)).map_err(|e| e.to_string())?;
        }

        execute!(self.stdout, MoveTo(cursor_pos.0, cursor_pos.1)).map_err(|e| e.to_string())?;

        self.stdout.flush().map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub fn clean_up() {
    disable_raw_mode().unwrap();
}
