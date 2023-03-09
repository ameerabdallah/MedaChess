enum Piece {
    ES, // Empty Square
    
    // White Pieces
    WK, // White King
    WQ, // White Queen
    WR, // White Rook
    WB, // White Bishop
    WN, // White Knight
    WP, // White Pawn

    // Black Pieces
    BK, // Black King
    BQ, // Black Queen
    BR, // Black Rook
    BB, // Black Bishop
    BN, // Black Knight
    BP,  // Black Pawn
}


// #[repr)]
// enum Score {
//     ScoreZero = 0 as i32,
//     Exact(i32)
// }

// const fn make_score(mg: i32, eg: i32) -> Score {
//     return Score::Exact((((eg as u32) << 16) as i32) + mg);
// }

// fn eg_value(score: Score) -> Value {

// }

// fn mg_value(score: Score) -> Value {

// }