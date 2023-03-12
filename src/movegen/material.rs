use crate::{
    bitboards::{
        types::{ Bitboard, MaskOrClear::* },
        tables::*,
        ranks::Rank::*,
        files::File::*,
        bitboards::Bitboards,
    },
    types::{ Color },
};

// The reason for not just using PieceType is because pawns require their color value
pub enum MovingPiece {
    Pawn(Color),
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Material {
    moving_piece: MovingPiece,
    location: Bitboard,
}

impl Material {
    pub fn new(moving_piece: MovingPiece, location: Bitboard) -> Self {
        Self {
            moving_piece,
            location,
        }
    }

    pub fn generate_moves(&self, bbs: &Bitboards) -> Bitboard {
        match self.moving_piece {
            MovingPiece::Pawn(color) => self.gen_pawn_moves(&color, bbs),
            MovingPiece::Knight => todo!(),
            MovingPiece::Bishop => todo!(),
            MovingPiece::Rook => todo!(),
            MovingPiece::Queen => todo!(),
            MovingPiece::King => todo!(),
        }
    }

    fn gen_pawn_moves(&self, color: &Color, bbs: &Bitboards) -> Bitboard {
        let all_pieces = bbs.get_all_pieces();
        match color {
            Color::White => self.gen_white_pawn_moves(all_pieces, bbs.get_pieces_bb(Color::Black)),
            Color::Black => self.gen_black_pawn_moves(all_pieces, bbs.get_pieces_bb(Color::White)),
        }
    }

    fn gen_white_pawn_moves(&self, all_pieces: Bitboard, black_pieces: Bitboard) -> Bitboard {
        // one step is a possible move if no pieces are in front of it
        let one_step = (self.location << 8) & !all_pieces;

        // adds 2 spaces in front if one_step is on rank 3 (this would mean it came from rank 2)
        let two_steps = ((one_step & RANK_BBS[(R3, Mask)]) << 8) & !all_pieces;

        let valid_moves = one_step | two_steps;

        // compute attacks for forward left and forward right while clipping if on A or H file
        let attacks =
            ((self.location & FILE_BBS[(A, Clear)]) << 7) |
            ((self.location & FILE_BBS[(H, Clear)]) << 9);

        // compute valid attacks by ANDing with black pieces
        let valid_attacks = attacks & black_pieces;

        // combine valid moves and valid attacks
        valid_moves | valid_attacks
    }

    fn gen_black_pawn_moves(&self, all_pieces: Bitboard, white_pieces: Bitboard) -> Bitboard {
        // one step is a possible move if no pieces are in front of it
        let one_step = (self.location >> 8) & !all_pieces;

        // adds 2 spaces in front if one_step is on rank 3 (this would mean it came from rank 2)
        let two_steps = ((one_step & RANK_BBS[(R6, Mask)]) >> 8) & !all_pieces;

        let valid_moves = one_step | two_steps;

        // compute attacks for forward left and forward right while clipping if on A or H file
        let attacks =
            ((self.location & FILE_BBS[(A, Clear)]) >> 9) |
            ((self.location & FILE_BBS[(H, Clear)]) >> 7);

        // compute valid attacks by ANDing with black pieces
        let valid_attacks = attacks & white_pieces;

        // combine valid moves and valid attacks
        valid_moves | valid_attacks
    }
}