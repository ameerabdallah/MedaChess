use strum::EnumCount;
use strum_macros::{ EnumIter, AsRefStr, EnumCount };

use crate::bitboards::{bitboards::Bitboards, bitboard::Bitboard};

pub const TOTAL_NUM_PIECES: usize = PieceType::COUNT * Color::COUNT;

#[derive(Copy, Clone, EnumIter, AsRefStr, EnumCount)]
pub enum Color {
    White = 0,
    Black = PieceType::COUNT as isize,
}

#[derive(Copy, Clone, EnumIter, AsRefStr, EnumCount)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[repr(u16)]
enum MoveType {
    NORMAL,
    PROMOTION = 1 << 14,
    ENPASSANT = 2 << 14,
    CASTLING = 3 << 14,
}