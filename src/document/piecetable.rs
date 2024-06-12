// DO NOT TOUCH THIS FILE UNKLESS IT IS ABSOLUTELY NECESARRY
// IT IS CURSED WITH BAD CODE
// STAY AWAY
// IT'S FOR YOUR OWN SAFETY

use std::cmp::Ordering;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Copy, Clone, Debug)]
enum PieceType {
    Added,
    Original,
}

struct Piece {
    piece_type: PieceType,
    start: usize,
    length: usize,
    newlines: Vec<usize>,
}

impl Piece {
    pub fn new(piece_type: PieceType, start: usize, length: usize, newlines: Vec<usize>) -> Self {
        Self {
            piece_type,
            start,
            length,
            newlines,
        }
    }
}

pub struct PieceTable {
    original: String,
    added: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(mut string: String) -> Self {
        if string.is_empty() {
            string = String::from("\n");
        }

        let mut newlines = count_newlines(&string);

        // the last line won't be returned if there isn't a newline at the end
        if string.as_bytes()[string.len() - 1] as char != '\n' {
            newlines.push(string.len() - 1);
        }

        let piece = Piece::new(PieceType::Original, 0, string.len(), newlines);

        Self {
            original: string,
            added: String::new(),
            pieces: vec![piece],
        }
    }

    // TODO:
    // - add some more error handling
    // - error handling when to or from is too big
    pub fn gen_string(&self, from: usize, to: usize) -> Result<Vec<String>, String> {
        if from > to {
            return Err(format![
                "gen_string: from (= {}) cannot be bigger than to (= {})",
                from, to
            ]);
        }

        let mut strings: Vec<String> = vec![String::new()];

        for piece in self.pieces.iter() {
            let buf = match piece.piece_type {
                PieceType::Added => &self.added,
                PieceType::Original => &self.original,
            };

            for (i, end) in piece.newlines.iter().enumerate() {
                let start = match i {
                    0 => piece.start,
                    _ => piece.newlines[i - 1] + piece.start,
                };

                let li = strings.len() - 1;
                strings[li].push_str(&buf[start..(*end + piece.start)]);
                strings.push(String::new());
            }

            let start = *piece.newlines.last().unwrap_or(&0) + piece.start;

            if start > piece.length + piece.start {
                continue;
            }

            let li = strings.len() - 1;
            strings[li].push_str(&buf[start..piece.length + piece.start]);
        }

        if strings.last().is_some_and(|s| s.is_empty()) {
            let _ = strings.pop();
        }

        if to > strings.len() {
            for _ in 0..(to - strings.len()) {
                strings.push(String::from("~\n"));
            }
        }

        // fuck this shit
        Ok(strings[from..to].to_owned())
    }

    pub fn gen_whole_string(&self) -> String {
        let mut string = String::new();

        for piece in self.pieces.iter() {
            let buf = match piece.piece_type {
                PieceType::Added => &self.added,
                PieceType::Original => &self.original,
            };

            string.push_str(&buf[piece.start..(piece.start + piece.length)]);
        }

        string
    }

    fn split_at(&mut self, index: usize) -> Result<usize, String> {
        let mut passed_size = 0;

        for (i, piece) in self.pieces.iter().enumerate() {
            if passed_size < index && index < passed_size + piece.length {
                let buf = match piece.piece_type {
                    PieceType::Added => &self.added,
                    PieceType::Original => &self.original,
                };

                let p1_len = index - passed_size;
                let p1_newlines = count_newlines(&buf[piece.start..(piece.start + p1_len)]);
                let p1 = Piece::new(piece.piece_type, piece.start, p1_len, p1_newlines);

                let p2_len = passed_size + piece.length - index;
                let p2_newlines =
                    count_newlines(&buf[(piece.start + p1_len)..(piece.start + p1_len + p2_len)]);
                let p2 = Piece::new(piece.piece_type, piece.start + p1_len, p2_len, p2_newlines);

                self.pieces[i] = p1;
                self.pieces.insert(i + 1, p2);

                return Ok(i + 1);
            } else if index == passed_size {
                return Ok(i);
            } else if index == passed_size + piece.length {
                return Ok(i + 1);
            }
            passed_size += piece.length;
        }

        println!("SPLIT AT: index = {}", index);
        self._print_table();

        Err(String::from("`split_at` failed!"))
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for p in &self.pieces {
            let buf = match p.piece_type {
                PieceType::Added => &self.added,
                PieceType::Original => &self.original,
            };

            let s = &buf[p.start..(p.start + p.length)];

            len += UnicodeSegmentation::graphemes(s, true).count();
        }
        len
    }

    pub fn newlines(&self) -> usize {
        let mut newlines = 0;
        for p in &self.pieces {
            newlines += p.newlines.len()
        }
        newlines
    }

    pub fn _insert(&mut self, index: usize, string: &str) -> Result<(), String> {
        let start_index = self.added.len();
        let newlines = count_newlines(string);

        self.added.push_str(string);

        let i = self.split_at(index)?;

        self.pieces.insert(
            i,
            Piece::new(
                PieceType::Added,
                start_index,
                self.added.len() - start_index,
                newlines,
            ),
        );

        Ok(())
    }

    pub fn insert_char(&mut self, index: usize, ch: char) -> Result<(), String> {
        let start_index = self.added.len();
        let newline_vec = if ch != '\n' { vec![] } else { vec![1] };

        self.added.push(ch);

        let i = self.split_at(index)?;

        self.pieces.insert(
            i,
            Piece::new(
                PieceType::Added,
                start_index,
                self.added.len() - start_index,
                newline_vec,
            ),
        );

        // inserting many times in the same place causes the piece table to have a bunch of 1 sized
        // pieces, this de-fragments them
        self.group_pieces(i);

        Ok(())
    }

    pub fn group_pieces(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let current_start = self.pieces[index].start;
        let current_length = self.pieces[index].length;

        let previous_start = self.pieces[index - 1].start;
        let previous_length = self.pieces[index - 1].length;

        if current_start == previous_start + previous_length {
            self.pieces.remove(index);
            let new_piece = &mut self.pieces[index - 1];
            new_piece.length += current_length;

            let string = &self.added[new_piece.start..new_piece.start + new_piece.length];
            let newlines = count_newlines(string);

            new_piece.newlines = newlines;
        }
    }

    pub fn delete(&mut self, index: usize) -> Result<(), String> {
        let i = self.split_at(index)?;

        let piece = &mut self.pieces[i];

        piece.length -= 1;
        piece.start += 1;

        // could be done smarter?
        let s = match piece.piece_type {
            PieceType::Added => &self.added[piece.start..(piece.start + piece.length)],
            PieceType::Original => &self.original[piece.start..(piece.start + piece.length)],
        };

        let newlines = count_newlines(s);

        piece.newlines = newlines;

        Ok(())
    }

    pub fn _delete_range(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from > to {
            return Err(format![
                "delete_range: from (= {}) cannot be bigger than to (= {})",
                from, to
            ]);
        }

        let i = self.split_at(to)? - 1;

        let leng = to - from;

        match self.pieces[i].length.cmp(&leng) {
            Ordering::Greater => {
                self.pieces[i].length -= leng;
            }
            Ordering::Equal => {
                self.pieces.remove(i);
            }
            Ordering::Less => {
                self.pieces.remove(i - 1);

                let len_before = (0..i).fold(0, |acc, x| acc + self.pieces[x].length);

                self._delete_range(from, to - len_before)?;
            }
        }

        Ok(())
    }

    pub fn get_line_length(&self, line: usize) -> usize {
        let mut passed_newlines = 0;
        let mut len = 0;

        for piece in self.pieces.iter() {
            for (i, newline_pos) in piece.newlines.iter().enumerate() {
                let start = match i {
                    0 => 0,
                    _ => piece.newlines[i - 1],
                };

                len += newline_pos - start;

                if passed_newlines == line {
                    return len;
                } else {
                    len = 0;
                }

                passed_newlines += 1;
            }

            let start = piece.newlines.last().unwrap_or(&0);

            if *start >= piece.length {
                continue;
            }

            len += piece.length - start;
        }

        len
    }

    pub fn get_absolute_pos(&self, nth: usize, line: usize) -> usize {
        let mut absolute_pos = nth;
        let mut temp_absolute_pos = nth;

        let mut passed_newlines = 0;

        for piece in self.pieces.iter() {
            for (i, newline_pos) in piece.newlines.iter().enumerate() {
                let start = match i {
                    0 => 0,
                    _ => piece.newlines[i - 1],
                };

                if passed_newlines == line {
                    return absolute_pos;
                }

                absolute_pos = temp_absolute_pos + newline_pos - start;
                temp_absolute_pos = absolute_pos;
                passed_newlines += 1;
            }

            let start = piece.newlines.last().unwrap_or(&0);

            if *start >= piece.length {
                continue;
            }

            temp_absolute_pos += piece.length - start;
        }

        absolute_pos
    }

    fn _print_table(&self) {
        println!("orignal buffer : {:?}", self.original);
        println!("added buffer   : {:?}", self.added);

        println!();

        println!("which - start_index - lenght - newlines");
        println!("---------------------------------------");
        for piece in self.pieces.iter() {
            println!(
                "{:?} - {} - {} - {:?}",
                piece.piece_type, piece.start, piece.length, piece.newlines,
            );
        }
        println!("---------------------------------------");
    }
}

fn count_newlines(string: &str) -> Vec<usize> {
    string
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(i, _)| i + 1)
        .collect()
}
