use std::cmp::Ordering;

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
    pub fn new(string: String) -> Self {
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

        let mut passed_newlines = 0;

        for piece in self.pieces.iter() {
            let new_newlines = passed_newlines + piece.newlines.len();

            // RETURNS TOO EARLY!!!!!
            /*
            if new_newlines <= from {
                passed_newlines = new_newlines;
                continue;
            }
            if passed_newlines >= to {
                return strings;
            }
            */

            let start = if passed_newlines < from && from < new_newlines {
                from - passed_newlines
            } else {
                0
            };

            let end = if passed_newlines < to && to < new_newlines {
                to - passed_newlines
            } else {
                piece.newlines.len()
            };

            let buf = match piece.piece_type {
                PieceType::Added => &self.added,
                PieceType::Original => &self.original,
            };

            let i = strings.len() - 1;

            let mut to_push = match start {
                0 => {
                    &buf[piece.start
                        ..(piece.start + maybe_get(&piece.newlines, start).unwrap_or(piece.length))]
                }
                _ => &buf[piece.newlines[start - 1]..(piece.start + piece.newlines[start])],
            };

            strings[i].push_str(to_push);

            for i in (start + 1)..end {
                to_push =
                    &buf[piece.start + piece.newlines[i - 1]..(piece.start + piece.newlines[i])];
                strings.push(to_push.to_string());
            }

            if !piece.newlines.is_empty()
                && piece.newlines[piece.newlines.len() - 1] != piece.length
            {
                to_push = &buf[(piece.start + piece.newlines[piece.newlines.len() - 1])
                    ..(piece.start + piece.length)];
                strings.push(to_push.to_string());
            }

            if to_push.ends_with('\n') {
                strings.push(String::new());
            }

            passed_newlines = new_newlines;
        }

        let before_last = &strings[strings.len() - 2];
        if before_last.chars().last().is_some_and(|c| c == '\n') {
            let _ = strings.pop();
        }

        if to > strings.len() {
            for _ in 0..(to - strings.len()) {
                strings.push(String::from("~"));
            }
        }

        // fuck this shit
        Ok(strings[from..to].to_owned())
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

    pub fn _len(&self) -> usize {
        let mut len = 0;
        for p in &self.pieces {
            len += p.length
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

    pub fn insert(&mut self, index: usize, string: &str) -> Result<(), String> {
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

    pub fn delete_range(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from > to {
            return Err(format![
                "delete_range: from (= {}) cannot be bigger than to (= {})",
                from, to
            ]);
        }

        let i = self.split_at(to)? - 1;

        self._print_table();

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

                self.delete_range(from, to - len_before)?;
            }
        }

        Ok(())
    }

    pub fn get_line_length(&self, line: usize) -> usize {
        let mut passed_newlines = 0;
        let mut len = 0;

        for piece in self.pieces.iter() {
            for (i, subpart) in piece.newlines.iter().enumerate() {
                let start = match i {
                    0 => 0,
                    _ => piece.newlines[i - 1],
                };

                len += subpart - start;
                passed_newlines += 1;

                if passed_newlines == line + 1 {
                    return len - 1;
                } else {
                    len = 0;
                }
            }

            let start = piece.newlines.last().unwrap_or(&0);
            if start + 1 >= piece.length {
                continue;
            }
            len += piece.length - start;
        }

        len - 1
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

fn maybe_get<T: Copy>(arr: &[T], i: usize) -> Option<T> {
    if i >= arr.len() {
        return None;
    }
    Some(arr[i])
}
