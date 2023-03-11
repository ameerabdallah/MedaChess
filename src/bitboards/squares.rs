use std::ops::Index;
use strum::IntoEnumIterator;
use strum_macros::{ EnumIter, AsRefStr };

use super::ranks::Rank;

use super::files::File;
use super::types::Bitboard;

#[derive(Copy, Clone, EnumIter, AsRefStr)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

const A1_BB: Bitboard = 0x0000000000000001;
const NUM_SQUARES: usize = 8 * 8;

impl Square {
    pub fn get_square(file: File, rank: Rank) -> Square {
        let file_index = file as usize;
        let rank_index = rank as usize;
        let square_index = rank_index * 8 + file_index;
        Square::iter()
            .nth(square_index)
            .unwrap_or_else(|| panic!("Invalid square index: {}", square_index))
    }
}

pub struct SquareBBs([Bitboard; 64]);

impl SquareBBs {
    pub const fn new() -> SquareBBs {
        let mut squares: SquareBBs = SquareBBs([0; NUM_SQUARES]);
        let mut i = 0;
        while i < NUM_SQUARES {
            squares.0[i] = A1_BB << i;
            i += 1;
        }
        squares
    }
}

impl Index<(File, Rank)> for SquareBBs {
    type Output = Bitboard;

    fn index(&self, file_rank: (File, Rank)) -> &Self::Output {
        let index = file_rank.1 as usize * 8 + file_rank.0 as usize;
        &self.0[index]
    }
}

impl Index<Square> for SquareBBs {
    type Output = Bitboard;

    fn index(&self, index: Square) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl Index<usize> for SquareBBs {
    type Output = Bitboard;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const H8_BB: Bitboard = 0x8000000000000000;

    #[test]
    fn test_square_bbs() {
        let square_bbs = SquareBBs::new();
        assert_eq!(square_bbs[Square::A1], A1_BB);
        assert_eq!(square_bbs[Square::H8], H8_BB);
    }

    #[test]
    fn test_square_bbs_index() {
        let square_bbs = SquareBBs::new();
        assert_eq!(square_bbs[(File::A, Rank::R1)], A1_BB);
        assert_eq!(square_bbs[(File::H, Rank::R8)], H8_BB);
        assert_eq!(square_bbs[Square::A1], A1_BB);
        assert_eq!(square_bbs[Square::H8], H8_BB);
        assert_eq!(square_bbs[0], A1_BB);
        assert_eq!(square_bbs[63], H8_BB);
    }
}