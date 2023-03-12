use bitboards::{ranks::Rank, files::File, bitboards::Bitboards, types::MaskOrClear::*};
use strum::IntoEnumIterator;
use types::{PieceType, Color};

use crate::bitboards::{bitboards::get_board_string, tables::{SQUARE_BBS, FILE_BBS, RANK_BBS}, squares::Square};
mod uci;
mod types;
mod bitboards;
mod movegen;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let bbs: Bitboards = Bitboards::new();

    for file in File::iter() {
        println!(
            "{} File\nmask:\n{}\nclear:\n{}",
            file.as_ref(),
            get_board_string(FILE_BBS[(file, Mask)]),
            get_board_string(FILE_BBS[(file, Clear)])
        );
        stdin.read_line(&mut input).unwrap();
        if input.eq("q\n") {
            break;
        }
    }
    input = String::new();
    for rank in Rank::iter() {
        println!(
            "{} Rank\nmask:\n{}\nclear:\n{}",
            rank.as_ref(),
            get_board_string(RANK_BBS.mask_rank(rank)),
            get_board_string(RANK_BBS.clear_rank(rank))
        );

        stdin.read_line(&mut input).unwrap();
        if input == "q\n" {
            break;
        }
    }
    input = String::new();
    for square in Square::iter() {
        println!(
            "{} Square:\n{}",
            square.as_ref(),
            get_board_string(SQUARE_BBS[square])
        );

        stdin.read_line(&mut input).unwrap();
        if input == "q\n" {
            break;
        }
    }
    input = String::new();
    for piece in PieceType::iter() {
        for color in Color::iter() {
            println!(
                "{} {}:\n{}",
                color.as_ref(),
                piece.as_ref(),
                get_board_string(bbs.get_bb(piece, color))
            );
            stdin.read_line(&mut input).unwrap();
            if input == "q\n" {
                break;
            }
        }
    }
    input = String::new();
}