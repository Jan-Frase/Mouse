use crate::backend::perft::run_perftree_debug;
use std::env;

mod backend;
mod constants;

fn main() {
    let args = env::args();
    run_perftree_debug(args);
}
