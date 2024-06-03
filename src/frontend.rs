use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

use crate::editor::{self, Editor};

pub struct Frontend {
    editor: Editor,
    stdout: Stdout,
    running: bool,
    input_pressed: bool,
}

impl Frontend {
    pub fn new(editor: Editor) -> Self {
        Self {
            editor,
            running: true,
            input_pressed: true,
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

        self.draw_text()?;
        while self.running {
            self.handle_input().map_err(|e| e.to_string())?;

            if self.input_pressed {
                self.draw_text()?;
            }

            self.input_pressed = false;
        }

        self.clean_up()?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<(), String> {
        self.input_pressed = true;

        let terminal_height = terminal::size().map_err(|e| e.to_string())?.1;

        if let Event::Key(key) = read().map_err(|e| e.to_string())? {
            if key.kind != KeyEventKind::Release {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('x')) => self.running = false,
                    #[rustfmt::skip]
                    (KeyModifiers::NONE, KeyCode::Up)    => self.editor.move_cursor(editor::Direction::Up, terminal_height),
                    #[rustfmt::skip]
                    (KeyModifiers::NONE, KeyCode::Down)  => self.editor.move_cursor(editor::Direction::Down, terminal_height),
                    #[rustfmt::skip]
                    (KeyModifiers::NONE, KeyCode::Left)  => self.editor.move_cursor(editor::Direction::Left, terminal_height),
                    #[rustfmt::skip]
                    (KeyModifiers::NONE, KeyCode::Right) => self.editor.move_cursor(editor::Direction::Right, terminal_height),
                    (KeyModifiers::NONE, KeyCode::Char(c))
                    | (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                        self.editor.insert(c)?;
                    }
                    (KeyModifiers::NONE, KeyCode::Enter)
                    | (KeyModifiers::SHIFT, KeyCode::Enter) => self.editor.insert_newline()?,
                    // TODO: add ctrl-delete
                    (KeyModifiers::NONE, KeyCode::Backspace)
                    | (KeyModifiers::SHIFT, KeyCode::Backspace) => self.editor.delete()?,
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.editor.save()?,
                    _ => self.input_pressed = false,
                }
            }
        }

        Ok(())
    }

    fn draw_text(&mut self) -> Result<(), String> {
        execute!(self.stdout, cursor::Hide, cursor::MoveTo(0, 0),).map_err(|e| e.to_string())?;

        let terminal_size = terminal::size().map_err(|e| e.to_string())?;
        let text = self.editor.get_text(terminal_size.1)?;

        for (offset, row) in text.iter().enumerate() {
            self.stdout
                .execute(Clear(ClearType::CurrentLine))
                .map_err(|e| e.to_string())?;

            self.stdout
                .write_all(row.as_bytes())
                .map_err(|e| e.to_string())?;

            execute!(self.stdout, MoveTo(0, offset as u16 + 1)).map_err(|e| e.to_string())?;
        }

        let cursor_pos = self.editor.get_cursor_pos();

        self.stdout.flush().map_err(|e| e.to_string())?;

        execute!(
            self.stdout,
            cursor::Show,
            MoveTo(cursor_pos.0, cursor_pos.1)
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn clean_up(&mut self) -> Result<(), String> {
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )
        .map_err(|e| e.to_string())?;

        disable_raw_mode().unwrap();

        Ok(())
    }
}
