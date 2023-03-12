use strum::EnumCount;

use crate::types::{ PieceType, Color, TOTAL_NUM_PIECES };

use super::bitboard::Bitboard;

pub struct Bitboards {
    // 0-5: white pieces
    // 6-11: black pieces
    all_bbs: [Bitboard; TOTAL_NUM_PIECES],

    white_pieces_bb: Bitboard, // all white pieces
    black_pieces_bb: Bitboard, // all black pieces
    all_pieces_bb: Bitboard, // all pieces
}

impl Bitboards {
    pub fn new() -> Bitboards {
        let mut bbs = Bitboards {
            all_bbs: [0; TOTAL_NUM_PIECES],
            white_pieces_bb: 0,
            black_pieces_bb: 0,
            all_pieces_bb: 0,
        };

        bbs.init();
        bbs
    }

    pub fn init(&mut self) {
        use PieceType::*;
        use Color::*;
        use super::initial_pos::*;

        self.all_bbs[Self::get_bb_index(King, White)] = WHITE_KINGS_BB;
        self.all_bbs[Self::get_bb_index(King, Black)] = BLACK_KINGS_BB;
        self.all_bbs[Self::get_bb_index(Queen, White)] = WHITE_QUEENS_BB;
        self.all_bbs[Self::get_bb_index(Queen, Black)] = BLACK_QUEENS_BB;
        self.all_bbs[Self::get_bb_index(Rook, White)] = WHITE_ROOKS_BB;
        self.all_bbs[Self::get_bb_index(Rook, Black)] = BLACK_ROOKS_BB;
        self.all_bbs[Self::get_bb_index(Bishop, White)] = WHITE_BISHOPS_BB;
        self.all_bbs[Self::get_bb_index(Bishop, Black)] = BLACK_BISHOPS_BB;
        self.all_bbs[Self::get_bb_index(Pawn, White)] = WHITE_PAWNS_BB;
        self.all_bbs[Self::get_bb_index(Pawn, Black)] = BLACK_PAWNS_BB;
        self.all_bbs[Self::get_bb_index(Knight, White)] = WHITE_KNIGHTS_BB;
        self.all_bbs[Self::get_bb_index(Knight, Black)] = BLACK_KNIGHTS_BB;

        self.white_pieces_bb = WHITE_BB;
        self.black_pieces_bb = BLACK_BB;
        self.all_pieces_bb = ALL_PIECES_BB;
    }

    pub fn update_bb(
        &mut self,
        piece: PieceType,
        color: Color,
        new_bitboard: Bitboard
    ) -> &mut Bitboards {
        self.all_bbs[Self::get_bb_index(piece, color)] = new_bitboard;

        match color {
            Color::White => self.update_all_white_pieces_bb(),
            Color::Black => self.update_all_black_pieces_bb(),
        }
        self
    }

    pub fn get_bb(&self, piece: PieceType, color: Color) -> Bitboard {
        self.all_bbs[Self::get_bb_index(piece, color)]
    }

    fn update_all_white_pieces_bb(&mut self) {
        self.white_pieces_bb = 0;
        for i in 0..PieceType::COUNT {
            self.white_pieces_bb |= self.all_bbs[i];
        }
        self.update_all_pieces_bb();
    }

    fn update_all_black_pieces_bb(&mut self) {
        self.black_pieces_bb = 0;
        for i in PieceType::COUNT..PieceType::COUNT * Color::COUNT {
            self.black_pieces_bb |= self.all_bbs[i];
        }
        self.update_all_pieces_bb();
    }

    fn update_all_pieces_bb(&mut self) {
        self.all_pieces_bb = self.white_pieces_bb | self.black_pieces_bb;
    }

    fn get_bb_index(piece: PieceType, color: Color) -> usize {
        (piece as usize) + (color as usize)
    }

    pub fn get_pieces_bb(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white_pieces_bb,
            Color::Black => self.black_pieces_bb,
        }
    }

    pub fn get_all_pieces(&self) -> Bitboard {
        self.all_pieces_bb
    }
}

pub fn get_board_string(b_board: Bitboard) -> String {
    println!("b_board: {:#x}", b_board);
    let mut board_string = String::new();
    let flipped_board = flip_board_vertical(b_board);
    for i in 0..64 {
        if (flipped_board & (1 << i)) != 0 {
            board_string.push('1');
        } else {
            board_string.push('.');
        }
        board_string.push(' ');
        if i % 8 == 7 {
            board_string.push('\n');
        }
    }
    board_string
}

fn flip_board_vertical(b_board: Bitboard) -> Bitboard {
    let mut flipped_board = b_board;
    let k1 = 0x00ff00ff00ff00ff;
    let k2 = 0x0000ffff0000ffff;
    flipped_board = ((flipped_board >> 8) & k1) | ((flipped_board & k1) << 8);
    flipped_board = ((flipped_board >> 16) & k2) | ((flipped_board & k2) << 16);
    flipped_board = (flipped_board >> 32) | (flipped_board << 32);
    flipped_board
}

#[cfg(test)]
mod tests {
    use crate::bitboards::bitboard::{Rank::*, Square::*, rank_bbs, square_bbs};

    use super::*;

    #[test]
    fn test_flip_board_vertical() {
        let mut bb_1 =  rank_bbs(Rank1) | square_bbs(B4);
        bb_1 = flip_board_vertical(bb_1);
        let mut bb_2 =  rank_bbs(Rank8) | square_bbs(B5);
        assert_eq!(bb_1, bb_2);
        bb_2 = flip_board_vertical(bb_2);
        bb_1 = flip_board_vertical(bb_1);
        assert_eq!(bb_1, bb_2);
    }

    #[test]
    fn test_board_initial_position() {
        // ensure that white pieces are symmetrically aligned with the black pieces across the center ranks
        let bbs = Bitboards::new();

        let white_pieces_bb = bbs.white_pieces_bb;
        let black_pieces_bb = bbs.black_pieces_bb;
        assert_eq!(white_pieces_bb, flip_board_vertical(black_pieces_bb));
        assert_eq!(flip_board_vertical(white_pieces_bb), black_pieces_bb);

        let white_pawns_bb = bbs.get_bb(PieceType::Pawn, Color::White);
        let black_pawns_bb = bbs.get_bb(PieceType::Pawn, Color::Black);
        assert_eq!(white_pawns_bb, flip_board_vertical(black_pawns_bb));
        assert_eq!(flip_board_vertical(white_pawns_bb), black_pawns_bb);

        let white_knights_bb = bbs.get_bb(PieceType::Knight, Color::White);
        let black_knights_bb = bbs.get_bb(PieceType::Knight, Color::Black);
        assert_eq!(white_knights_bb, flip_board_vertical(black_knights_bb));
        assert_eq!(flip_board_vertical(white_knights_bb), black_knights_bb);

        let white_bishops_bb = bbs.get_bb(PieceType::Bishop, Color::White);
        let black_bishops_bb = bbs.get_bb(PieceType::Bishop, Color::Black);
        assert_eq!(white_bishops_bb, flip_board_vertical(black_bishops_bb));
        assert_eq!(flip_board_vertical(white_bishops_bb), black_bishops_bb);

        let white_rooks_bb = bbs.get_bb(PieceType::Rook, Color::White);
        let black_rooks_bb = bbs.get_bb(PieceType::Rook, Color::Black);
        assert_eq!(white_rooks_bb, flip_board_vertical(black_rooks_bb));
        assert_eq!(flip_board_vertical(white_rooks_bb), black_rooks_bb);

        let white_queens_bb = bbs.get_bb(PieceType::Queen, Color::White);
        let black_queens_bb = bbs.get_bb(PieceType::Queen, Color::Black);
        assert_eq!(white_queens_bb, flip_board_vertical(black_queens_bb));
        assert_eq!(flip_board_vertical(white_queens_bb), black_queens_bb);

        let white_king_bb = bbs.get_bb(PieceType::King, Color::White);
        let black_king_bb = bbs.get_bb(PieceType::King, Color::Black);
        assert_eq!(white_king_bb, flip_board_vertical(black_king_bb));
        assert_eq!(flip_board_vertical(white_king_bb), black_king_bb);

        // ensure that pawns are on the correct ranks (only need to test white since we already tested symmetry)
        assert_eq!(white_pawns_bb & rank_bbs(Rank2), rank_bbs(Rank2));

        // ensure remaining pieces occupy their corresponding files (once again, symmetry is already tested)
        assert_eq!(white_knights_bb & rank_bbs(Rank1), square_bbs(B1) | square_bbs(G1));
        assert_eq!(white_bishops_bb & rank_bbs(Rank1), square_bbs(C1) | square_bbs(F1));
        assert_eq!(white_rooks_bb & rank_bbs(Rank1), square_bbs(A1) | square_bbs(H1));
        assert_eq!(white_queens_bb & rank_bbs(Rank1), square_bbs(D1));
        assert_eq!(white_king_bb & rank_bbs(Rank1), square_bbs(E1));

        // ensure all white pieces occupy the ranks 1 and 2
        assert_eq!(white_pieces_bb & rank_bbs(Rank1) | rank_bbs(Rank2), rank_bbs(Rank1) | rank_bbs(Rank2));
    }
}