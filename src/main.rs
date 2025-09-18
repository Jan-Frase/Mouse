use backend::move_gen_king;

mod backend;
mod constants;

fn main() {
    println!("Hello, world!");
    let king_moves = move_gen_king::KING_MOVES[0];
    println!("{king_moves}");
}
