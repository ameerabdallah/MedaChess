
use bitboards::{bitboards::Bitboards, bitboard::{File, Rank, Square}};
use strum::IntoEnumIterator;
use types::{PieceType, Color};

use crate::bitboards::{bitboard::{file_bbs, rank_bbs, square_bbs}, bitboards::get_board_string};
mod uci;
mod types;
mod bitboards;
mod movegen;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let bbs: Bitboards = Bitboards::new();

    for file in File::iter() {
        match file {
            File::Nb => break,
            _ => {
                println!(
                    "{}:\n{}",
                    file.as_ref(),
                    get_board_string(file_bbs(file))
                );
            }
        }
        stdin.read_line(&mut input).unwrap();
    }
    
    for rank in Rank::iter() {
        match rank {
            Rank::Nb => break,
            _ => {
                println!(
                    "{}:\n{}",
                    rank.as_ref(),
                    get_board_string(rank_bbs(rank))
                );
            }
        }
        stdin.read_line(&mut input).unwrap();
    }
    
    for square in Square::iter() {
        match square {
            Square::Nb => break,
            _ => {
                println!(
                    "{}:\n{}",
                    square.as_ref(),
                    get_board_string(square_bbs(square))
                );
            }
        }
        stdin.read_line(&mut input).unwrap();
    }

    for piece in PieceType::iter() {
        for color in Color::iter() {
            println!(
                "{} {}:\n{}",
                color.as_ref(),
                piece.as_ref(),
                get_board_string(bbs.get_bb(piece, color))
            );
        }
    }
}