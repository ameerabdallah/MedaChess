use strum_macros::{AsRefStr, EnumIter};

pub(crate) type Bitboard = u64;

pub enum Direction {
    North,
    NorthNorth, // for white pawn moves
    South,
    SouthSouth, // for black pawn moves
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

impl Direction {
    pub fn shift(self, bb: Bitboard) -> Bitboard {
        match self {
            Direction::North => bb << 8,
            Direction::South => bb >> 8,
            Direction::NorthNorth => bb << 16,
            Direction::SouthSouth => bb >> 16,
            Direction::East => (bb & !FILE_HBB) << 1,
            Direction::West => (bb & !FILE_ABB) >> 1,
            Direction::NorthEast => (bb & !FILE_HBB) << 9,
            Direction::NorthWest => (bb & !FILE_ABB) << 7,
            Direction::SouthEast => (bb & !FILE_HBB) >> 7,
            Direction::SouthWest => (bb & !FILE_ABB) >> 9,
        }
    }
}

#[cfg(test)]
mod direction_test {
    use super::*;

    #[test]
    fn test_shift() {
        assert_eq!(Direction::North.shift(square_bbs(Square::A1)), square_bbs(Square::A2));
        assert_eq!(Direction::South.shift(square_bbs(Square::A2)), square_bbs(Square::A1));
        assert_eq!(Direction::NorthNorth.shift(square_bbs(Square::A1)), square_bbs(Square::A3));
        assert_eq!(Direction::SouthSouth.shift(square_bbs(Square::A3)), square_bbs(Square::A1));
        assert_eq!(Direction::East.shift(square_bbs(Square::A1)), square_bbs(Square::B1));
        assert_eq!(Direction::West.shift(square_bbs(Square::B1)), square_bbs(Square::A1));
        assert_eq!(Direction::NorthEast.shift(square_bbs(Square::A1)), square_bbs(Square::B2));
        assert_eq!(Direction::SouthEast.shift(square_bbs(Square::A2)), square_bbs(Square::B1));
        assert_eq!(Direction::NorthWest.shift(square_bbs(Square::H1)), square_bbs(Square::G2));
        assert_eq!(Direction::SouthWest.shift(square_bbs(Square::H2)), square_bbs(Square::G1));
        
        // cuts off the edge
        assert_eq!(Direction::NorthEast.shift(square_bbs(Square::H1)), 0);
        assert_eq!(Direction::NorthWest.shift(square_bbs(Square::A1)), 0);
        
        assert_eq!(Direction::SouthEast.shift(square_bbs(Square::H2)), 0);
        assert_eq!(Direction::SouthWest.shift(square_bbs(Square::A2)), 0);
        
        assert_eq!(Direction::North.shift(square_bbs(Square::A8)), 0);
        assert_eq!(Direction::South.shift(square_bbs(Square::A1)), 0);
        
        assert_eq!(Direction::NorthNorth.shift(square_bbs(Square::A8)), 0);
        assert_eq!(Direction::NorthNorth.shift(square_bbs(Square::A7)), 0);
        assert_eq!(Direction::SouthSouth.shift(square_bbs(Square::A1)), 0);
        assert_eq!(Direction::SouthSouth.shift(square_bbs(Square::A2)), 0);
        
        assert_eq!(Direction::East.shift(square_bbs(Square::H5)), 0);
        assert_eq!(Direction::West.shift(square_bbs(Square::A4)), 0);
    }
}


#[derive(Copy, Clone, EnumIter, AsRefStr)]
#[repr(usize)]
pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Nb = 8
}

type RankArray = [Bitboard; Rank::Nb as usize];

// Masks
pub const RANK1_BB: Bitboard = 0x00000000000000FF;
pub const RANK2_BB: Bitboard = RANK1_BB << (8 * 1);
pub const RANK3_BB: Bitboard = RANK1_BB << (8 * 2);
pub const RANK4_BB: Bitboard = RANK1_BB << (8 * 3);
pub const RANK5_BB: Bitboard = RANK1_BB << (8 * 4);
pub const RANK6_BB: Bitboard = RANK1_BB << (8 * 5);
pub const RANK7_BB: Bitboard = RANK1_BB << (8 * 6);
pub const RANK8_BB: Bitboard = RANK1_BB << (8 * 7);

pub const RANK_BBS: RankArray = [
    RANK1_BB,
    RANK2_BB,
    RANK3_BB,
    RANK4_BB,
    RANK5_BB,
    RANK6_BB,
    RANK7_BB,
    RANK8_BB,
];

pub const fn rank_bbs(rank: Rank) -> Bitboard {
    RANK_BBS[rank as usize]
}

#[cfg(test)]
mod rank_test {
    use super::*;

    #[test]
    fn test_rank_bbs() {
        assert_eq!(rank_bbs(Rank::Rank1), RANK1_BB);
        assert_eq!(rank_bbs(Rank::Rank2), RANK2_BB);
        assert_eq!(rank_bbs(Rank::Rank3), RANK3_BB);
        assert_eq!(rank_bbs(Rank::Rank4), RANK4_BB);
        assert_eq!(rank_bbs(Rank::Rank5), RANK5_BB);
        assert_eq!(rank_bbs(Rank::Rank6), RANK6_BB);
        assert_eq!(rank_bbs(Rank::Rank7), RANK7_BB);
        assert_eq!(rank_bbs(Rank::Rank8), RANK8_BB);

        assert_eq!(rank_bbs(Rank::Rank1), RANK_BBS[0]);
        assert_eq!(rank_bbs(Rank::Rank2), RANK_BBS[1]);
        assert_eq!(rank_bbs(Rank::Rank3), RANK_BBS[2]);
        assert_eq!(rank_bbs(Rank::Rank4), RANK_BBS[3]);
        assert_eq!(rank_bbs(Rank::Rank5), RANK_BBS[4]);
        assert_eq!(rank_bbs(Rank::Rank6), RANK_BBS[5]);
        assert_eq!(rank_bbs(Rank::Rank7), RANK_BBS[6]);
        assert_eq!(rank_bbs(Rank::Rank8), RANK_BBS[7]);
    }
}

#[derive(Copy, Clone, EnumIter, AsRefStr)]
#[repr(usize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    Nb = 8,
}

// Masks
pub const FILE_ABB: Bitboard = 0x0101010101010101;
pub const FILE_BBB: Bitboard = FILE_ABB << 1;
pub const FILE_CBB: Bitboard = FILE_ABB << 2;
pub const FILE_DBB: Bitboard = FILE_ABB << 3;
pub const FILE_EBB: Bitboard = FILE_ABB << 4;
pub const FILE_FBB: Bitboard = FILE_ABB << 5;
pub const FILE_GBB: Bitboard = FILE_ABB << 6;
pub const FILE_HBB: Bitboard = FILE_ABB << 7;

type FileArray = [Bitboard; File::Nb as usize];

pub const FILE_BBS: FileArray = [
    FILE_ABB,
    FILE_BBB,
    FILE_CBB,
    FILE_DBB,
    FILE_EBB,
    FILE_FBB,
    FILE_GBB,
    FILE_HBB,
];

pub const fn file_bbs(file: File) -> Bitboard {
    FILE_BBS[file as usize]
}

#[cfg(test)]
mod file_test {
    use super::*;

    #[test]
    fn test_file_bbs() {
        assert_eq!(file_bbs(File::A), FILE_ABB);
        assert_eq!(file_bbs(File::B), FILE_BBB);
        assert_eq!(file_bbs(File::C), FILE_CBB);
        assert_eq!(file_bbs(File::D), FILE_DBB);
        assert_eq!(file_bbs(File::E), FILE_EBB);
        assert_eq!(file_bbs(File::F), FILE_FBB);
        assert_eq!(file_bbs(File::G), FILE_GBB);
        assert_eq!(file_bbs(File::H), FILE_HBB);

        assert_eq!(file_bbs(File::A), FILE_BBS[0]);
        assert_eq!(file_bbs(File::B), FILE_BBS[1]);
        assert_eq!(file_bbs(File::C), FILE_BBS[2]);
        assert_eq!(file_bbs(File::D), FILE_BBS[3]);
        assert_eq!(file_bbs(File::E), FILE_BBS[4]);
        assert_eq!(file_bbs(File::F), FILE_BBS[5]);
        assert_eq!(file_bbs(File::G), FILE_BBS[6]);
        assert_eq!(file_bbs(File::H), FILE_BBS[7]);
    }
}

#[derive(Copy, Clone, EnumIter, AsRefStr)]
#[repr(usize)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    Nb = 64
}

const fn gen_mask_squares() -> [Bitboard; Square::Nb as usize] {
    let mut squares = [0; Square::Nb as usize];
    let mut i = Square::A1 as usize;
    while i < Square::Nb as usize{
        squares[i] = A1_BB << i;
        i += 1;
    }
    squares
}

const A1_BB: Bitboard = 0x0000000000000001;

type SquareArray = [Bitboard; Square::Nb as usize];

pub const SQUARE_BBS: SquareArray = gen_mask_squares();

pub const fn square_bbs(square: Square) -> Bitboard {
    SQUARE_BBS[square as usize]
}
pub const fn square_bbs_2d(file: File, rank: Rank) -> Bitboard {
    SQUARE_BBS[((rank as usize) << 3) + file as usize]
}

#[cfg(test)]
mod square_test {
    use super::*;
    const H8_BB: Bitboard = 0x8000000000000000; // not indexing to make sure that the index works properly in the test_square_bbs_index test

    #[test]
    fn test_bb_index() {
        assert_eq!(square_bbs_2d(File::A, Rank::Rank1), A1_BB);
        assert_eq!(square_bbs_2d(File::H, Rank::Rank8), H8_BB);
        assert_eq!(square_bbs(Square::A1), A1_BB);
        assert_eq!(square_bbs(Square::H8), H8_BB);
        assert_eq!(SQUARE_BBS[0], A1_BB);
        assert_eq!(SQUARE_BBS[63], H8_BB);
    }
}

pub const QUEEN_SIDE: Bitboard      = FILE_ABB | FILE_BBB | FILE_CBB | FILE_DBB;
pub const KING_SIDE: Bitboard       = FILE_EBB | FILE_FBB | FILE_GBB | FILE_HBB;
pub const CENTER_FILES: Bitboard    = FILE_CBB | FILE_DBB | FILE_EBB | FILE_FBB;
pub const CENTER: Bitboard          = (FILE_DBB | FILE_EBB) & (RANK4_BB | RANK5_BB);