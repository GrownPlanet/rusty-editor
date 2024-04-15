use bevy::ecs::system::Resource;

use crate::{document::Document, frontend};

#[derive(Resource)]
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

        frontend::run_frontend(screen_dimensions, title, self)
    }

    // TODO: optimize this function so it only gets the text that fits on screen
    pub fn get_text(&self) -> String {
        self.document.get_text()
    }

    pub fn get_cursor_pos(&self) -> (u32, u32) {
        self.document.get_cursor_pos()
    }
}
