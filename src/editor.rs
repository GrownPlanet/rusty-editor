use std::{fs::{self, File}, path::{Path, PathBuf}};

use crate::piecetable::PieceTable;

pub struct Editor {
    piece_table: PieceTable,
    file: File
}

impl Editor {
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
            piece_table, 
            file,
        })
    }
}
