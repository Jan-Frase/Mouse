struct PerftSetUp {
    fen: &'static str,
    name: &'static str,
}

const STARTING_POS: PerftSetUp = PerftSetUp {
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ",
    name: "starting pos",
};

const POSITION_2: PerftSetUp = PerftSetUp {
    fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
    name: "position 2",
};

const POSITION_3: PerftSetUp = PerftSetUp {
    fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    name: "position 3",
};

const POSITION_4: PerftSetUp = PerftSetUp {
    fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    name: "position 4",
};

const POSITION_5: PerftSetUp = PerftSetUp {
    fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    name: "position 5",
};

const POSITION_6: PerftSetUp = PerftSetUp {
    fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    name: "position 6",
};

const KING_ONLY: PerftSetUp = PerftSetUp {
    fen: "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
    name: "king only",
};

const KING_AND_ROOK: PerftSetUp = PerftSetUp {
    fen: "1n2k1n1/8/8/8/8/8/8/1N2K1N1 w - - 0 1",
    name: "king and rook",
};

const KING_AND_KNIGHT: PerftSetUp = PerftSetUp {
    fen: "1n2k1n1/8/8/8/8/8/8/1N2K1N1 w - - 0 1",
    name: "king and knight",
};

const KING_AND_BISHOP: PerftSetUp = PerftSetUp {
    fen: "2b1kb2/8/8/8/8/8/8/2B1KB2 w - - 0 1",
    name: "king and bishop",
};

const KING_AND_QUEEN: PerftSetUp = PerftSetUp {
    fen: "3qk3/8/8/8/8/8/8/3QK3 w - - 0 11",
    name: "king and queen",
};

struct PerftSetUpExtended {
    perft_setup: PerftSetUp,
    depth: u8,
    expected_nodes: u64,
}

const STARTING_POS_FAST: PerftSetUpExtended = PerftSetUpExtended {
    perft_setup: STARTING_POS,
    depth: 2,
    expected_nodes: 400,
};
