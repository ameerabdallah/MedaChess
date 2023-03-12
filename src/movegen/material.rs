use crate::{
    types::{ Color }, bitboards::{bitboard::{Bitboard, Direction, RANK2_BB, RANK7_BB}, bitboards::Bitboards},
};

// The reason for not just using PieceType is because pawns require their color value
// since pawn moves are determined by color
pub enum MovingPiece {
    Pawn(Color),
    Knight(Bitboard),
    Bishop(Bitboard),
    Rook(Bitboard),
    Queen(Bitboard),
    King(Bitboard),
}

pub struct Material {
    moving_piece: MovingPiece,
    own_side: Bitboard,
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
            MovingPiece::Knight(location) => todo!(),
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
        let one_step = Direction::North.shift(self.location) & !all_pieces;

        // adds 2 spaces in front if one_step is on rank 3 (this would mean it came from rank 2)
        let two_steps = Direction::NorthNorth.shift(self.location & RANK2_BB) & !all_pieces;

        let valid_moves = one_step | two_steps;

        // compute attacks for forward left and forward right while clipping if on A or H file
        let attacks = Direction::NorthEast.shift(self.location) | Direction::NorthWest.shift(self.location);

        // compute valid attacks by ANDing with black pieces
        let valid_attacks = attacks & black_pieces;

        // combine valid moves and valid attacks
        valid_moves | valid_attacks
    }

    fn gen_black_pawn_moves(&self, all_pieces: Bitboard, white_pieces: Bitboard) -> Bitboard {
        // one step is a possible move if no pieces are in front of it
        let one_step = Direction::South.shift(self.location) & !all_pieces;

        // adds 2 spaces in front if one_step is on rank 3 (this would mean it came from rank 2)
        let two_steps = Direction::SouthSouth.shift(self.location & RANK7_BB) & !all_pieces;

        let valid_moves = one_step | two_steps;

        // compute attacks for forward left and forward right while clipping if on A or H file
        let attacks = Direction::SouthEast.shift(self.location) | Direction::SouthWest.shift(self.location);

        // compute valid attacks by ANDing with white pieces
        let valid_attacks = attacks & white_pieces;

        // combine valid moves and valid attacks
        valid_moves | valid_attacks
    }

    fn gen_king_moves(&self, all_pieces: Bitboard, white_pieces: Bitboard) -> Bitboard {
        let one_step = Direction::North.shift(self.location) | Direction::South.shift(self.location) | Direction::East.shift(self.location) | Direction::West.shift(self.location) | Direction::NorthEast.shift(self.location) | Direction::NorthWest.shift(self.location) | Direction::SouthEast.shift(self.location) | Direction::SouthWest.shift(self.location);
    }
}
