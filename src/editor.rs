use std::process::exit;

use crate::{document::Document, frontend};

pub struct Editor {
    document: Document,
    // I am first going to make a working text editor
    // config: Config,
}

impl Editor {
    pub fn new(path: &str) -> Result<Self, String> {
        let document = Document::new(path)?;

        Ok(Self { document })
    }

    pub fn run(self) {
        let screen_dimensions = (800, 600);
        let title = "rusty text editor";

        if let Err(s) = frontend::run_frontend(self) {
            println!("Error: {}", s);
            exit(-1);
        }
    }

    // TODO: optimize this function so it only gets the text that fits on screen
    pub fn get_text(&self) -> String {
        self.document.get_text()
    }

    pub fn get_cursor_pos(&self) -> (u32, u32) {
        self.document.get_cursor_pos()
    }
}
