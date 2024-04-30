// This could be made faster
// future reference:
// https://code.visualstudio.com/blogs/2018/03/23/text-buffer-reimplementation#_boost-line-lookup-by-using-a-balanced-binary-tree

use std::cmp::Ordering;

// is it from the add buffer or the original buffer
#[derive(Debug, Clone, Copy)]
enum Part {
    Add,
    Original,
}

// a single piece from the piece table
struct Piece {
    which: Part,
    start_index: usize,
    length: usize,
    newlines: Vec<usize>,
}

impl Piece {
    pub fn new(which: Part, start_index: usize, length: usize, newlines: Vec<usize>) -> Self {
        Self {
            which,
            start_index,
            length,
            newlines,
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
        let newlines = Self::count_newlines(&string);

        let piece = Piece::new(Part::Original, 0, string.len(), newlines);

        Self {
            original: string,
            add: String::new(),
            table: vec![piece],
        }
    }

    // generate a string from the table
    pub fn generate_string(&self, from: usize, to: usize) -> Result<Vec<String>, String> {
        if from > to {
            return Err(format![
                "from (= {}) cannot be bigger than to (= {})",
                from, to
            ]);
        }

        // TODO
        /*

           0                        3
        o: [    |          |     |  ]
           3            5
        a: [      |..|..]
           5             7
        a: [.....|....|..]
           7     8
        o: [..|..]
           8                                    12
        o: [.....|      |           |      |    ]

        4 - 9

        */

        let generated_string = vec![];

        let mut passed_newlines = 0;

        for piece in self.table.iter() {
            let start_index = piece.start_index;
            let end_index = piece.start_index + piece.length;

            let s = match &piece.which {
                Part::Add => &self.add[start_index..end_index],
                Part::Original => &self.original[start_index..end_index],
            };

            // generated_string.push();

            passed_newlines += piece.newlines.len();
        }

        Ok(generated_string)
    }

    // split the table at a given index
    fn split_at(&mut self, index: usize) -> Result<usize, String> {
        let mut passed_size = 0;
        for (i, piece) in self.table.iter().enumerate() {
            if index > passed_size && index < passed_size + piece.length {
                let p1_len = index - passed_size;
                let p2_len = (passed_size + piece.length) - index;

                // TODO

                let part1 = Piece::new(piece.which, piece.start_index, p1_len, vec![]);
                let part2 = Piece::new(piece.which, piece.start_index + p1_len, p2_len, vec![]);

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

        let newlines = Self::count_newlines(string);

        self.table.insert(
            i,
            Piece::new(
                Part::Add,
                start_index,
                self.add.len() - start_index,
                newlines,
            ),
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

    fn count_newlines(string: &str) -> Vec<usize> {
        let mut newlines = vec![];

        for (i, c) in string.chars().enumerate() {
            if c == '\n' {
                newlines.push(i);
            }
        }

        newlines
    }
}
