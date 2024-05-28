use crate::document::Document;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

    // TODO: optimize this function so it only gets the text that fits on screen
    pub fn get_text(&self, terminal_height: u16) -> Result<Vec<String>, String> {
        let from = self.scroll_off;
        let to = from + terminal_height;

        self.document.get_text(0, to as usize)
    }

    pub fn get_cursor_pos(&self) -> (u16, u16) {
        self.cursor_pos
    }

    pub fn move_cursor(&mut self, direction: Direction, terminal_height: u16) {
        // moving the cursor
        match direction {
            Direction::Up => self.cursor_pos.1 = self.cursor_pos.1.saturating_sub(1),
            Direction::Down => self.cursor_pos.1 += 1,
            Direction::Left => self.cursor_pos.0 = self.cursor_pos.0.saturating_sub(1),
            Direction::Right => self.cursor_pos.0 += 1,
        }

        // clamping the position
        let max = std::cmp::min(
            terminal_height - 1,
            self.document.len() as u16 - self.scroll_off - 1,
        );
        self.cursor_pos.1 = self.cursor_pos.1.clamp(0, max);

        // `- 1` because we don't want to insert after the newline
        let max = self.document.line_len(self.cursor_pos.1 as usize) as u16 - 1;
        self.cursor_pos.0 = self.cursor_pos.0.clamp(0, max);
    }

    pub fn insert(&mut self, ch: char) -> Result<(), String> {
        self.document
            .insert(ch, self.cursor_pos.0 as usize, self.cursor_pos.1 as usize)?;

        self.cursor_pos.0 += 1;

        Ok(())
    }
}
