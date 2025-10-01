use crate::backend::state::piece::PieceType;
use crate::backend::state::piece::PieceType::{Pawn, Queen};

pub fn get_pawn_quiet_moves() {}

pub fn get_pawn_capture_moves() {}

pub fn get_pawn_double_push_moves() {}

fn promotion_logic() {
    if piece_type == Pawn && !moves.is_empty() {
        for index in moves.len() - 1..=0 {
            let moove = moves[index];
            if moove.to().is_on_promotion_rank() {
                for piece_type in PieceType::get_promotable_types() {
                    if piece_type == Queen {
                        moves[index].set_promotion_type(Some(Queen));
                        continue;
                    }
                    let mut moove = moove.clone();
                    moove.set_promotion_type(Some(piece_type));
                    moves.push(moove);
                }
            }
        }
    }
}
