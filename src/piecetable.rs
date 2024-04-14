use std::cmp::Ordering;

// is it from the add buffer or the original buffer
#[derive(Debug, Clone, Copy)]
enum Which {
    Add,
    Original,
}

// a single piece from the piece table
struct Piece {
    which: Which,
    start_index: usize,
    length: usize,
}

impl Piece {
    pub fn new(which: Which, start_index: usize, length: usize) -> Self {
        Self {
            which,
            start_index,
            length,
        }
    }
}

pub struct PieceTable {
    original: String,
    add: String,
    table: Vec<Piece>,
}

impl PieceTable {
    // create a new table
    pub fn new(string: String) -> Self {
        let piece = Piece::new(Which::Original, 0, string.len());
        Self {
            original: string,
            add: String::new(),
            table: vec![piece],
        }
    }

    // generate a string from the table
    pub fn generate_string(&self) -> String {
        let mut generated_string = String::new();

        for piece in self.table.iter() {
            let s = match &piece.which {
                Which::Add => &self.add[piece.start_index..(piece.start_index + piece.length)],
                Which::Original => {
                    &self.original[piece.start_index..(piece.start_index + piece.length)]
                }
            };
            generated_string.push_str(s);
        }

        generated_string
    }

    // split the table at a given index
    fn split_at(&mut self, index: usize) -> Result<usize, String> {
        let mut passed_size = 0;
        for (i, piece) in self.table.iter().enumerate() {
            if index > passed_size && index < passed_size + piece.length {
                let p1_len = index - passed_size;
                let part1 = Piece::new(piece.which, piece.start_index, p1_len);
                let p2_len = (passed_size + piece.length) - index;
                let part2 = Piece::new(piece.which, piece.start_index + p1_len, p2_len);

                self.table[i] = part1;
                self.table.insert(i + 1, part2);

                return Ok(i + 1); // plus one because we need to insert there
            } else if index == passed_size {
                return Ok(i);
            } else if index == passed_size + piece.length {
                return Ok(i + 1);
            }
            passed_size += piece.length;
        }

        Err(String::from("split_at failed!"))
    }

    // insert a string at a given index
    pub fn insert(&mut self, index: usize, string: &str) -> Result<(), String> {
        let start_index = self.add.len();

        self.add.push_str(string);
        let i = self.split_at(index)?;

        self.table.insert(
            i,
            Piece::new(Which::Add, start_index, self.add.len() - start_index),
        );

        Ok(())
    }

    // for debugging purposes
    pub fn _print_table(&self) {
        println!("which - start_index - lenght");
        println!("----------------------------");
        for piece in self.table.iter() {
            println!(
                "{:?} - {} - {}",
                piece.which, piece.start_index, piece.length
            );
        }
        println!("----------------------------");
    }

    // delete a char at a given index
    pub fn delete(&mut self, index: usize) -> Result<(), String> {
        let i = self.split_at(index)?;

        self.table[i].length -= 1;
        self.table[i].start_index += 1;

        Ok(())
    }

    // delete a range of chars
    pub fn delete_range(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from > to {
            return Err(format![
                "from (= {}) cannot be bigger than to (= {})",
                from, to
            ]);
        }

        let i = self.split_at(to)? - 1;
        let l = to - from;

        match self.table[i].length.cmp(&l) {
            Ordering::Less => {
                self.table[i].length -= l;
            }
            Ordering::Equal => {
                self.table.remove(i);
            }
            Ordering::Greater => {
                self.table.remove(i);
                self.delete_range(from, to - self.table[i].length)?;
            }
        }

        Ok(())
    }

    // get the lenght of the piece table
    pub fn len(&self) -> usize {
        let mut len = 0;
        for piece in self.table.iter() {
            len += piece.length;
        }
        len
    }
}
