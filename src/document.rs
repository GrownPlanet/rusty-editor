use std::{
    fs::{self, File},
    io::Write,
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
            // TODO: don't delete the file when just opening it :/
            let file = File::create(path).map_err(|e| e.to_string())?;
            (content, file)
        } else {
            let content = String::from("\n");
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

    pub fn line_len(&self, line: usize) -> usize {
        self.piece_table.get_line_length(line)
    }

    pub fn insert(&mut self, ch: char, nth: usize, line: usize) -> Result<(), String> {
        let absolute_pos = self.piece_table.get_absolute_pos(nth, line);
        self.piece_table.insert(absolute_pos, &ch.to_string())?;

        Ok(())
    }

    pub fn delete(&mut self, nth: usize, line: usize) -> Result<(), String> {
        let absolute_pos = self.piece_table.get_absolute_pos(nth, line);
        self.piece_table.delete(absolute_pos)?;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        // clear the file before writing to it again
        self.file.set_len(0).map_err(|e| e.to_string())?;

        let string = self
            .piece_table
            .gen_whole_string();

        self.file.write_all(&string.as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }
}
