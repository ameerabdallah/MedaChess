use bitboards::bitboard::{File, FileBBs, Rank, RankBBs, Square, SquareBBs, Bitboards, Piece, Color};
use strum::IntoEnumIterator;

use crate::bitboards::bitboard::get_board_string;
mod uci;
mod types;
mod bitboards;

// pub static OPTIONS: HashMap<String, String> = HashMap::new();

fn main() {
    let stdin = std::io::stdin();
    let file_bbs = FileBBs::new();
    let rank_bbs = RankBBs::new();
    let square_bbs = SquareBBs::new();
    let bbs = Bitboards::new();
    let mut input = String::new();

    for file in File::iter() {
        println!("{} File\nmask:\n{}\nclear:\n{}", 
            file.as_ref(),
            get_board_string(file_bbs.get_mask_file(file)),
            get_board_string(file_bbs.get_clear_file(file)));
        stdin.read_line(&mut input).unwrap();
        if input.eq("q\n") {
            break;
        }
    }
    input = String::new();
    for rank in Rank::iter() {
        println!("{} Rank\nmask:\n{}\nclear:\n{}", 
            rank.as_ref(),
            get_board_string(rank_bbs.get_mask_rank(rank)),
            get_board_string(rank_bbs.get_clear_rank(rank)));
        
        stdin.read_line(&mut input).unwrap();
        if input == "q\n" {
            break;
        }
    }
    input = String::new();
    for square in Square::iter() {
        println!("{} Square:\n{}", 
            square.as_ref(),
            get_board_string(square_bbs.get_square_bb(square)));
        
        stdin.read_line(&mut input).unwrap();
        if input == "q\n" {
            break;
        }
    }
    input = String::new();
    for piece in Piece::iter() {
        for color in Color::iter() {
            println!("{} {}:\n{}", 
                color.as_ref(),
                piece.as_ref(),
                get_board_string(bbs.get_bb(piece, color)));
            stdin.read_line(&mut input).unwrap();
            if input == "q\n" {
                break;
            }
        }
    }
    input = String::new();


}
