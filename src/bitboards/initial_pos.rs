use super::bitboard::{rank_bbs, Rank, Bitboard};


const RANK_2_TO_7: u8 = 8 * 5; // 8 bits per rank, 5 ranks between 2 and 7
const RANK_1_TO_8: u8 = 8 * 7; // 8 bits per rank, 7 ranks between 1 and 8

pub const WHITE_PAWNS_BB: Bitboard = rank_bbs(Rank::Rank2);
pub const BLACK_PAWNS_BB: Bitboard = WHITE_PAWNS_BB << RANK_2_TO_7;
pub const WHITE_KNIGHTS_BB: Bitboard = 0x0000000000000042;
pub const BLACK_KNIGHTS_BB: Bitboard = WHITE_KNIGHTS_BB << RANK_1_TO_8;
pub const WHITE_BISHOPS_BB: Bitboard = 0x0000000000000024;
pub const BLACK_BISHOPS_BB: Bitboard = WHITE_BISHOPS_BB << RANK_1_TO_8;
pub const WHITE_ROOKS_BB: Bitboard = 0x0000000000000081;
pub const BLACK_ROOKS_BB: Bitboard = WHITE_ROOKS_BB << RANK_1_TO_8;
pub const WHITE_QUEENS_BB: Bitboard = 0x0000000000000008;
pub const BLACK_QUEENS_BB: Bitboard = WHITE_QUEENS_BB << RANK_1_TO_8;
pub const WHITE_KINGS_BB: Bitboard = 0x0000000000000010;
pub const BLACK_KINGS_BB: Bitboard = WHITE_KINGS_BB << RANK_1_TO_8;

pub const WHITE_BB: Bitboard =
    WHITE_PAWNS_BB |
    WHITE_KNIGHTS_BB |
    WHITE_BISHOPS_BB |
    WHITE_ROOKS_BB |
    WHITE_QUEENS_BB |
    WHITE_KINGS_BB;

pub const BLACK_BB: Bitboard =
    BLACK_PAWNS_BB |
    BLACK_KNIGHTS_BB |
    BLACK_BISHOPS_BB |
    BLACK_ROOKS_BB |
    BLACK_QUEENS_BB |
    BLACK_KINGS_BB;

pub const ALL_PIECES_BB: Bitboard = WHITE_BB | BLACK_BB;