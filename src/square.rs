/// This represents a square on the chess board.
/// The square A1 is at file == 0 and rank == 0.
/// The square H1 is at file == 7 and rank == 0.
///
/// To make it easier to memorize: file => the letter part, rank => the number part
/// or put differently: file => vertical / x part, rank => horizontal / y part
#[derive(Copy, Clone)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub(crate) fn square_to_index(&self) -> u8 {
        self.file + self.rank * 8
    }
}
