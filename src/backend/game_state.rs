use crate::backend::bitboard_manager::BitBoardManager;

pub struct GameState {
    bit_board_manager: BitBoardManager,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            bit_board_manager: BitBoardManager::new(),
        }
    }

    pub fn bit_board_manager(&mut self) -> &mut BitBoardManager {
        &mut self.bit_board_manager
    }
}
