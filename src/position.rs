struct StateInfo {
    pawn_key: u64,
    material_key: u64,
    non_pawn_material: [i32; 2],
    castling_rights: u8,
    rule_50: i32,
    plies_from_null: i32,
}

struct Position {
    board: Board,
    side_to_move: Color,
    state: StateInfo,
    zobrist: Zobrist,
    history: Vec<StateInfo>,
}

impl Position {
    fn from_fen(fen: str) -> Position {
        
    }
}