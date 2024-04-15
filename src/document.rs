use std::{
    fs::{self, File},
    path::Path,
};

use self::piecetable::PieceTable;

mod piecetable;

pub struct Document {
    cursor: (u32, u32),
    piece_table: PieceTable,
    file: File,
}

impl Document {
    pub fn new(path: &str) -> Result<Self, String> {
        let path = Path::new(path);

        let (file_contents, file) = if path.exists() {
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let file = File::open(path).map_err(|e| e.to_string())?;
            (content, file)
        } else {
            let content = String::new();
            let file = File::create(path).map_err(|e| e.to_string())?;
            (content, file)
        };

        let piece_table = PieceTable::new(file_contents);

        Ok(Self {
            cursor: (0, 0),
            piece_table,
            file,
        })
    }

    pub fn get_text(&self) -> String {
        self.piece_table.generate_string()
    }

    pub fn get_cursor_pos(&self) -> (u32, u32) {
        self.cursor
    }
}
