use crate::backend::bitboard::BitBoard;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::constants::{PIECE_TYPE_COUNT, SIDES};

pub struct BitBoardManager {
    bitboards: [BitBoard; PIECE_TYPE_COUNT * SIDES],
    bitboard_index_to_piece: [Piece; PIECE_TYPE_COUNT * SIDES],
}

impl BitBoardManager {
    pub fn new() -> BitBoardManager {
        let bitboards = [BitBoard::new(); PIECE_TYPE_COUNT * SIDES];

        let mut bitboard_index_to_piece =
            [Piece::new(PieceType::Pawn, PieceColor::White); PIECE_TYPE_COUNT * SIDES];

        // insert all the white pieces
        bitboard_index_to_piece[0] = Piece::new(PieceType::Pawn, PieceColor::White);
        bitboard_index_to_piece[1] = Piece::new(PieceType::Rook, PieceColor::White);
        bitboard_index_to_piece[2] = Piece::new(PieceType::Knight, PieceColor::White);
        bitboard_index_to_piece[3] = Piece::new(PieceType::Bishop, PieceColor::White);
        bitboard_index_to_piece[4] = Piece::new(PieceType::Queen, PieceColor::White);
        bitboard_index_to_piece[5] = Piece::new(PieceType::King, PieceColor::White);

        // insert all the black pieces
        bitboard_index_to_piece[6] = Piece::new(PieceType::Pawn, PieceColor::Black);
        bitboard_index_to_piece[7] = Piece::new(PieceType::Rook, PieceColor::Black);
        bitboard_index_to_piece[8] = Piece::new(PieceType::Knight, PieceColor::Black);
        bitboard_index_to_piece[9] = Piece::new(PieceType::Bishop, PieceColor::Black);
        bitboard_index_to_piece[10] = Piece::new(PieceType::Queen, PieceColor::Black);
        bitboard_index_to_piece[11] = Piece::new(PieceType::King, PieceColor::Black);

        BitBoardManager {
            bitboards,
            bitboard_index_to_piece,
        }
    }

    fn piece_to_bitboards_index(&self, piece: Piece) -> usize {
        let mut index = 0;

        // this causes the index to be at 0 for white and 6 for black
        // it works because white as usize == 0 and black as usize == 1
        index += piece.piece_color() as usize * PIECE_TYPE_COUNT;
        // this adds 0 for a pawn, 1 for a rook, etc.
        index += piece.piece_type() as usize;
        // the indices are
        // white: pawn: 0, rook: 1, knight: 2, bishop: 3, queen: 4, king: 5
        // black: pawn: 6, rook: 7, knight: 8, bishop: 9, queen: 10, king: 11
        index
    }

    fn bitboard_index_to_piece(&self, index: usize) -> Piece {
        self.bitboard_index_to_piece[index]
    }
}
