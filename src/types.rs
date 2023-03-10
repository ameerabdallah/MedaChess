use strum::{EnumCount};
use strum_macros::{ EnumIter, AsRefStr, EnumCount };

pub const TOTAL_NUM_PIECES: usize = Piece::COUNT * Color::COUNT;

#[derive(Copy, Clone, EnumIter, AsRefStr, EnumCount)]
pub enum Color {
    White = 0,
    Black = Piece::COUNT as isize,
}

#[derive(Copy, Clone, EnumIter, AsRefStr, EnumCount)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}