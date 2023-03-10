use std::iter;

use super::bb_utils::Bitboard;

struct Bitboards {
    // 0-5: white pieces
    // 6-11: black pieces
    all_bbs: [Bitboard; 12],

    all_white_pieces_bb: Bitboard, // all white pieces
    all_black_pieces_bb: Bitboard, // all black pieces
    all_pieces_bb: Bitboard   // all pieces
}

impl Bitboards {
    fn update_bb(&mut self, piece: Piece, color: Color, new_bitboard: Bitboard) {
        self.all_bbs[Self::get_bb_index(piece, color)] = new_bitboard;

        match color {
            Color::White => self.update_all_white_pieces_bb(),
            Color::Black => self.update_all_black_pieces_bb(),
        };
    }

    fn update_all_white_pieces_bb(&mut self) {
        self.all_white_pieces_bb = 0;
        for i in 0..6 {
            self.all_white_pieces_bb |= self.all_bbs[i];
        }
        self.update_all_pieces_bb();
    }

    fn update_all_black_pieces_bb(&mut self) {
        self.all_black_pieces_bb = 0;
        for i in 6..12 {
            self.all_black_pieces_bb |= self.all_bbs[i];
        }
        self.update_all_pieces_bb();
    }

    fn update_all_pieces_bb(&mut self) {
        self.all_pieces_bb = self.all_white_pieces_bb | self.all_black_pieces_bb;
    }

    fn get_bb_index(piece: Piece, color: Color) -> usize {
        piece as usize + color as usize
    }
}

#[derive(Copy, Clone)]
enum Color {
    White = 0,
    Black = 6
}

#[derive(Copy, Clone)]
#[repr(usize)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone)]
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

struct FileBBs {
    mask_files: [Bitboard; 8],
    clear_files: [Bitboard; 8],
}

impl FileBBs {
    fn new() -> FileBBs {
        use super::bb_utils::files::*;

        FileBBs {
            mask_files: [
                MASK_FILE_A_BB,
                MASK_FILE_B_BB,
                MASK_FILE_C_BB,
                MASK_FILE_D_BB,
                MASK_FILE_E_BB,
                MASK_FILE_F_BB,
                MASK_FILE_G_BB,
                MASK_FILE_H_BB,
            ],
            clear_files: [
                CLEAR_FILE_A_BB,
                CLEAR_FILE_B_BB,
                CLEAR_FILE_C_BB,
                CLEAR_FILE_D_BB,
                CLEAR_FILE_E_BB,
                CLEAR_FILE_F_BB,
                CLEAR_FILE_G_BB,
                CLEAR_FILE_H_BB,
            ]
        }
    }

    fn get_mask_file(&self, file: File) -> Bitboard {
        self.mask_files[file as usize]
    }

    fn get_clear_file(&self, file: File) -> Bitboard {
        self.clear_files[file as usize]
    }
}

enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
}

struct RankBBs {
    mask_ranks: [Bitboard; 8],
    clear_ranks: [Bitboard; 8],
}

impl RankBBs {
    fn new() -> RankBBs {
        use super::bb_utils::ranks::*;

        RankBBs {
            mask_ranks: [
                MASK_RANK1_BB,
                MASK_RANK2_BB,
                MASK_RANK3_BB,
                MASK_RANK4_BB,
                MASK_RANK5_BB,
                MASK_RANK6_BB,
                MASK_RANK7_BB,
                MASK_RANK8_BB,
            ],
            clear_ranks: [
                CLEAR_RANK1_BB,
                CLEAR_RANK2_BB,
                CLEAR_RANK3_BB,
                CLEAR_RANK4_BB,
                CLEAR_RANK5_BB,
                CLEAR_RANK6_BB,
                CLEAR_RANK7_BB,
                CLEAR_RANK8_BB,
            ]
        }
    }

    fn get_mask_rank(&self, rank: Rank) -> Bitboard {
        self.mask_ranks[rank as usize]
    }

    fn get_clear_rank(&self, rank: Rank) -> Bitboard {
        self.clear_ranks[rank as usize]
    }
}

#[derive(Copy, Clone)]
enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

struct SquareBBs {
    squares: [Bitboard; 64],
}

impl SquareBBs {
    fn new() -> SquareBBs {
        use super::bb_utils::A1;
        let mut squares: [Bitboard; 64] = [0; 64];
        for i in 0..64 {
            squares[i] = A1 << i as u64;
        }
        SquareBBs {
            squares
        }
    }

    fn get_square_bb(&self, square: Square) -> Bitboard {
        self.squares[square as usize]
    }
}