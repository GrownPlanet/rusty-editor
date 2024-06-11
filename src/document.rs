use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use self::piecetable::PieceTable;

mod piecetable;

pub struct Document {
    piece_table: PieceTable,
    path: String,
}

impl Document {
    pub fn new(path: &str) -> Result<Self, String> {
        let filepath = Path::new(path);

        let file_contents = if filepath.exists() {
            fs::read_to_string(filepath).map_err(|e| e.to_string())?
        } else {
            String::from("\n")
        };

        let piece_table = PieceTable::new(file_contents);

        Ok(Self {
           piece_table,
           path: path.to_string(),
        })
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
        // don't delete the last newline
        // I probably should allow it to open files that don't end with newlines
        // meh, it works kind of for now
        if self.piece_table.len() <= 1 {
            return Ok(());
        }

        let absolute_pos = self.piece_table.get_absolute_pos(nth, line);

        self.piece_table.delete(absolute_pos)?;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), String> {
        // clear the file before writing to it again
        let mut file = File::create(&self.path).map_err(|e| e.to_string())?;

        let string = self.piece_table.gen_whole_string();

        file.write_all(string.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
