use std::collections::HashMap;

// Bit flags for the pegs
struct Peg;
impl Peg {
    pub const P0: u16 = 1;
    pub const P1: u16 = 2;
    pub const P2: u16 = 4;
    pub const P3: u16 = 8;
    pub const P4: u16 = 16;
    pub const P5: u16 = 32;
    pub const P6: u16 = 64;
    pub const P7: u16 = 128;
    pub const P8: u16 = 256;
    pub const P9: u16 = 512;
    pub const P10: u16 = 1024;
    pub const P11: u16 = 2048;
    pub const P12: u16 = 4096;
    pub const P13: u16 = 8192;
    pub const P14: u16 = 16384;
    pub const P15: u16 = 32768;
}

/// Solver for a 15 peg gameboard.
/// Peg positions:
///        0
///      1  2
///     3  4  5
///   6  7  8  9
/// 10 11 12 13 14
fn main() {
    let now = std::time::Instant::now();
    let possible_moves = generate_possible_moves();
    let mut games = 0;

    // Start with a "full board" with all bits set (u16::MAX)
    // Remove Peg 15 (which does not exist)
    // Remove the starting peg (there are only 4 unique options)
    println!("No Peg 0 (corner)");
    games += launch_game(u16::MAX - Peg::P15 - Peg::P0, &possible_moves);

    println!("No Peg 1 (side, one in)");
    games += launch_game(u16::MAX - Peg::P15 - Peg::P1, &possible_moves);

    println!("No Peg 3 (middle side)");
    games += launch_game(u16::MAX - Peg::P15 - Peg::P3, &possible_moves);

    println!("No Peg 4 (third row middle)");
    games += launch_game(u16::MAX - Peg::P15 - Peg::P4, &possible_moves);

    println!("Total games: {games}");
    println!("Total time:  {:?}", now.elapsed());
}

/// Structure for all the runs, results counting, etc
fn launch_game(board: u16, possible_moves: &Vec<(u16, u16, u16)>) -> u32 {
    let mut results: HashMap<u32, u32> = HashMap::new();

    play_game(board, possible_moves, &mut results);

    // Sort the results so it is easier to read
    let mut sorted: Vec<_> = results.iter().collect();
    sorted.sort_by_key(|a| a.0);

    // Print out the game counts by ending pegs and total game count
    let mut games = 0;
    for (key, val) in &sorted {
        println!("  Pegs: {key}, Games: {val}");
        games += *val;
    }
    println!("  Total games: {games}");
    games
}

/// Find possible moves and play all of them recursively, when no moves, record final peg count
fn play_game(board: u16, possible_moves: &Vec<(u16, u16, u16)>, results: &mut HashMap<u32, u32>) {
    let mut moves: Vec<(u16, u16, u16)> = Vec::new();

    find_moves(board, &mut moves, possible_moves);
    for (start, skip, end) in &moves {
        play_game(make_move(board, *start, *skip, *end), possible_moves, results);
    }

    // If this game is done, record the peg count
    if moves.is_empty() {
        *results.entry(board.count_ones()).or_default() += 1;
    }
}

/// Find available moves for a given board.
fn find_moves(board: u16, moves: &mut Vec<(u16, u16, u16)>, possible_moves: &Vec<(u16, u16, u16)>) {
    for (start, skip, end) in possible_moves {
        // Use bitwise operators to find conditions where a move is possible
        // (only look at three pegs) == (are the first two pegs, and the final open?)
        if board & (start + skip + end) == (start + skip) {
            moves.push((*start, *skip, *end));
        }
    }
}

/// Make a move and return the board
fn make_move(board: u16, start: u16, skip: u16, end: u16) -> u16 {
    // Start and skip go away, end appears
    board - start - skip + end
}

/// Create a vector of possible moves (start, skip, end)
fn generate_possible_moves() -> Vec<(u16, u16, u16)> {
    let moves: Vec<(u16, u16, u16)> = vec![
        (Peg::P0, Peg::P1, Peg::P3),
        (Peg::P0, Peg::P2, Peg::P5),
        (Peg::P1, Peg::P3, Peg::P6),
        (Peg::P1, Peg::P4, Peg::P8),
        (Peg::P2, Peg::P4, Peg::P7),
        (Peg::P2, Peg::P5, Peg::P9),
        (Peg::P3, Peg::P1, Peg::P0),
        (Peg::P3, Peg::P4, Peg::P5),
        (Peg::P3, Peg::P7, Peg::P12),
        (Peg::P3, Peg::P6, Peg::P10),
        (Peg::P4, Peg::P7, Peg::P11),
        (Peg::P4, Peg::P8, Peg::P13),
        (Peg::P5, Peg::P2, Peg::P0),
        (Peg::P5, Peg::P4, Peg::P3),
        (Peg::P5, Peg::P9, Peg::P14),
        (Peg::P5, Peg::P8, Peg::P12),
        (Peg::P6, Peg::P3, Peg::P1),
        (Peg::P6, Peg::P7, Peg::P8),
        (Peg::P7, Peg::P4, Peg::P2),
        (Peg::P7, Peg::P8, Peg::P9),
        (Peg::P8, Peg::P7, Peg::P6),
        (Peg::P8, Peg::P4, Peg::P1),
        (Peg::P9, Peg::P5, Peg::P2),
        (Peg::P9, Peg::P7, Peg::P8),
        (Peg::P10, Peg::P6, Peg::P3),
        (Peg::P10, Peg::P11, Peg::P12),
        (Peg::P11, Peg::P7, Peg::P4),
        (Peg::P11, Peg::P12, Peg::P13),
        (Peg::P12, Peg::P7, Peg::P3),
        (Peg::P12, Peg::P11, Peg::P10),
        (Peg::P12, Peg::P13, Peg::P14),
        (Peg::P12, Peg::P8, Peg::P5),
        (Peg::P13, Peg::P12, Peg::P11),
        (Peg::P13, Peg::P8, Peg::P4),
        (Peg::P14, Peg::P9, Peg::P5),
        (Peg::P14, Peg::P13, Peg::P12),
    ];
    moves
}
