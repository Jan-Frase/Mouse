// Jan: I have started working on this, but im not sure if this is any good so far lol.

pub struct Piece {
    value: u8,
}

enum PieceTypeValue {
    Pawn = 1,
    Rook = 2,
    Knight = 4,
    Bishop = 8,
    Queen = 16,
    King = 32,
}

enum SideValue {
    White = 64,
    Black = 128,
}

impl Piece {
    fn new(piece_value: PieceTypeValue, side_value: SideValue) -> Piece {
        let value: u8 = piece_value as u8 | side_value as u8;
        Piece { value }
    }

    pub fn get_piece_type(&self) -> u8 {
        todo!()
    }
}
