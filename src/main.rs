use bitboards::bitboard::{File, FileBBs, Rank, RankBBs, Square, SquareBBs};
mod uci;
mod types;
mod bitboards;

// pub static OPTIONS: HashMap<String, String> = HashMap::new();

fn main() {

    let file_bbs = FileBBs::new();
    let rank_bbs = RankBBs::new();
    let square_bbs = SquareBBs::new();
    println!("Hello, world!");
}
