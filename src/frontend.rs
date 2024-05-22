use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
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
            // if self.input_pressed {
                self.draw_text()?;
            // }

            self.input_pressed = false;
        }

        clean_up();

        Ok(())
    }

    fn handle_input(&mut self) -> std::io::Result<()> {
        self.input_pressed = true;

        if let Event::Key(key) = read()? {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => self.running = false,
                #[rustfmt::skip]
                (KeyModifiers::NONE, KeyCode::Up)    => self.editor.move_cursor(editor::Direction::Up),
                #[rustfmt::skip]
                (KeyModifiers::NONE, KeyCode::Down)  => self.editor.move_cursor(editor::Direction::Down),
                #[rustfmt::skip]
                (KeyModifiers::NONE, KeyCode::Left)  => self.editor.move_cursor(editor::Direction::Left),
                #[rustfmt::skip]
                (KeyModifiers::NONE, KeyCode::Right) => self.editor.move_cursor(editor::Direction::Right),
                _ => self.input_pressed = false,
            }
        }

        Ok(())
    }

    fn draw_text(&mut self) -> Result<(), String> {
        execute!(self.stdout, cursor::Hide, cursor::MoveTo(0, 0),).map_err(|e| e.to_string())?;

        let terminal_size = terminal::size().map_err(|e| e.to_string())?;
        let text = self.editor.get_text(terminal_size.1)?;

        for (offset, row) in text.iter().enumerate() {
            self.stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine));
            
            self.stdout
                .write_all(row.as_bytes())
                .map_err(|e| e.to_string())?;
            // print!("{}", row);

            // std::thread::sleep(std::time::Duration::from_millis(50));

            execute!(self.stdout, MoveTo(0, offset as u16)).map_err(|e| e.to_string())?;

            // std::thread::sleep(std::time::Duration::from_millis(50));
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
}

pub fn clean_up() {
    disable_raw_mode().unwrap();
}
