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

    pub fn run(&mut self) {
        let screen_dimensions = (800, 600);
        let title = "rusty text editor";

        frontend::run_frontend(screen_dimensions, title)
    }
}
