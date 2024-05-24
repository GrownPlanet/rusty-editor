use std::{
    fs::{self, File},
    path::Path,
};

use self::piecetable::PieceTable;

mod piecetable;

pub struct Document {
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

        Ok(Self { piece_table, file })
    }

    pub fn get_text(&self, from: usize, to: usize) -> Result<Vec<String>, String> {
        self.piece_table.gen_string(from, to)
    }

    pub fn len(&self) -> usize {
        self.piece_table.newlines()
    }
}
