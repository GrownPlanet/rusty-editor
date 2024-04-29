use std::process::exit;

use crate::{document::Document, frontend::Frontend};

pub struct Editor {
    document: Document,
    cursor_pos: (u16, u16),
    scroll_off: u16,
    // I am first going to make a working text editor
    // config: Config,
}

impl Editor {
    pub fn new(path: &str) -> Result<Self, String> {
        let document = Document::new(path)?;

        Ok(Self {
            document,
            cursor_pos: (0, 0),
            scroll_off: 0,
        })
    }

    pub fn run(self) {
        let mut frontend = Frontend::new(self);

        if let Err(s) = frontend.run() {
            println!("Error: {}", s);
            exit(-1);
        }
    }

    // TODO: optimize this function so it only gets the text that fits on screen
    pub fn get_text(&self, terminal_height: u16) -> Result<Vec<String>, String> {
        let from = self.scroll_off + self.cursor_pos.0;
        let to = from + terminal_height;

        self.document.get_text(from as usize, to as usize)
    }

    pub fn get_cursor_pos(&self) -> (u16, u16) {
        self.cursor_pos
    }
}
