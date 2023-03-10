pub(crate) type Bitboard = u64;

// bitboard format being used is LSB = A1, MSB = H8

pub(super) mod files {
    use super::Bitboard;

    pub const MASK_FILE_A_BB: Bitboard = 0x101010101010101;
    pub const MASK_FILE_B_BB: Bitboard = MASK_FILE_A_BB << 8;
    pub const MASK_FILE_C_BB: Bitboard = MASK_FILE_B_BB << 8;
    pub const MASK_FILE_D_BB: Bitboard = MASK_FILE_C_BB << 8;
    pub const MASK_FILE_E_BB: Bitboard = MASK_FILE_D_BB << 8;
    pub const MASK_FILE_F_BB: Bitboard = MASK_FILE_E_BB << 8;
    pub const MASK_FILE_G_BB: Bitboard = MASK_FILE_F_BB << 8;
    pub const MASK_FILE_H_BB: Bitboard = MASK_FILE_G_BB << 8;
    
    pub const CLEAR_FILE_A_BB: Bitboard = !MASK_FILE_A_BB;
    pub const CLEAR_FILE_B_BB: Bitboard = !MASK_FILE_B_BB;
    pub const CLEAR_FILE_C_BB: Bitboard = !MASK_FILE_C_BB;
    pub const CLEAR_FILE_D_BB: Bitboard = !MASK_FILE_D_BB;
    pub const CLEAR_FILE_E_BB: Bitboard = !MASK_FILE_E_BB;
    pub const CLEAR_FILE_F_BB: Bitboard = !MASK_FILE_F_BB;
    pub const CLEAR_FILE_G_BB: Bitboard = !MASK_FILE_G_BB;
    pub const CLEAR_FILE_H_BB: Bitboard = !MASK_FILE_H_BB;
}

pub(super) mod ranks {
    use super::Bitboard;

    pub const MASK_RANK1_BB: Bitboard = 0x00000000000000FF;
    pub const MASK_RANK2_BB: Bitboard = MASK_RANK1_BB << 8;
    pub const MASK_RANK3_BB: Bitboard = MASK_RANK2_BB << 8;
    pub const MASK_RANK4_BB: Bitboard = MASK_RANK3_BB << 8;
    pub const MASK_RANK5_BB: Bitboard = MASK_RANK4_BB << 8;
    pub const MASK_RANK6_BB: Bitboard = MASK_RANK5_BB << 8;
    pub const MASK_RANK7_BB: Bitboard = MASK_RANK6_BB << 8;
    pub const MASK_RANK8_BB: Bitboard = MASK_RANK7_BB << 8;

    pub const CLEAR_RANK1_BB: Bitboard = !MASK_RANK1_BB;
    pub const CLEAR_RANK2_BB: Bitboard = !MASK_RANK2_BB;
    pub const CLEAR_RANK3_BB: Bitboard = !MASK_RANK3_BB;
    pub const CLEAR_RANK4_BB: Bitboard = !MASK_RANK4_BB;
    pub const CLEAR_RANK5_BB: Bitboard = !MASK_RANK5_BB;
    pub const CLEAR_RANK6_BB: Bitboard = !MASK_RANK6_BB;
    pub const CLEAR_RANK7_BB: Bitboard = !MASK_RANK7_BB;
    pub const CLEAR_RANK8_BB: Bitboard = !MASK_RANK8_BB;
}

pub(super) mod initial_positions {
    use super::Bitboard;

    const RANK_2_TO_7: u8 = 8 * 5; // 8 bits per rank, 5 ranks between 2 and 7
    const RANK_1_TO_8: u8 = 8 * 7; // 8 bits per rank, 7 ranks between 1 and 8

    pub const INITIAL_WHITE_PAWN_BB: Bitboard = 0x000000000000FF00;
    pub const INITIAL_BLACK_PAWN_BB: Bitboard = INITIAL_WHITE_PAWN_BB << RANK_2_TO_7;
    pub const INITIAL_WHITE_KNIGHT_BB: Bitboard = 0x0000000000000042;
    pub const INITIAL_BLACK_KNIGHT_BB: Bitboard = INITIAL_WHITE_KNIGHT_BB << RANK_1_TO_8;
    pub const INITIAL_WHITE_BISHOP_BB: Bitboard = 0x0000000000000024;
    pub const INITIAL_BLACK_BISHOP_BB: Bitboard = INITIAL_WHITE_BISHOP_BB << RANK_1_TO_8;
    pub const INITIAL_WHITE_ROOK_BB: Bitboard = 0x0000000000000081;
    pub const INITIAL_BLACK_ROOK_BB: Bitboard = INITIAL_WHITE_ROOK_BB << RANK_1_TO_8;
    pub const INITIAL_WHITE_QUEEN_BB: Bitboard = 0x0000000000000008;
    pub const INITIAL_BLACK_QUEEN_BB: Bitboard = INITIAL_WHITE_QUEEN_BB << RANK_1_TO_8;
    pub const INITIAL_WHITE_KING_BB: Bitboard = 0x0000000000000010;
    pub const INITIAL_BLACK_KING_BB: Bitboard = INITIAL_WHITE_KING_BB << RANK_1_TO_8;
}

pub(super) const A1: Bitboard = 0x0000000000000001;