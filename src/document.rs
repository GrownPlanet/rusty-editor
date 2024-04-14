use crate::piecetable::PieceTable;

pub struct Document {
    cursor: (u32, u32),
    piece_table: PieceTable,
}

impl Document {
    pub fn new(source: String) -> Self {
        let piece_table = PieceTable::new(source);

        Self {
            cursor: (0, 0),
            piece_table,
        }
    }
}
