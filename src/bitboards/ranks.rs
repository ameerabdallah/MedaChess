use std::{ops::Index};

use strum_macros::{EnumIter, AsRefStr};

use super::types::{Bitboard, MaskOrClear};


#[derive(Copy, Clone, EnumIter, AsRefStr)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

// Masks
const MASK_RANK1_BB: Bitboard = 0x00000000000000ff;
const MASK_RANK2_BB: Bitboard = MASK_RANK1_BB << 8;
const MASK_RANK3_BB: Bitboard = MASK_RANK2_BB << 8;
const MASK_RANK4_BB: Bitboard = MASK_RANK3_BB << 8;
const MASK_RANK5_BB: Bitboard = MASK_RANK4_BB << 8;
const MASK_RANK6_BB: Bitboard = MASK_RANK5_BB << 8;
const MASK_RANK7_BB: Bitboard = MASK_RANK6_BB << 8;
const MASK_RANK8_BB: Bitboard = MASK_RANK7_BB << 8;

// Clears
const CLEAR_RANK1_BB: Bitboard = !MASK_RANK1_BB;
const CLEAR_RANK2_BB: Bitboard = !MASK_RANK2_BB;
const CLEAR_RANK3_BB: Bitboard = !MASK_RANK3_BB;
const CLEAR_RANK4_BB: Bitboard = !MASK_RANK4_BB;
const CLEAR_RANK5_BB: Bitboard = !MASK_RANK5_BB;
const CLEAR_RANK6_BB: Bitboard = !MASK_RANK6_BB;
const CLEAR_RANK7_BB: Bitboard = !MASK_RANK7_BB;
const CLEAR_RANK8_BB: Bitboard = !MASK_RANK8_BB;

pub struct RankBBs {
    mask_ranks: [Bitboard; 8],
    clear_ranks: [Bitboard; 8],
}

impl RankBBs {
    pub const fn new() -> RankBBs {
        RankBBs {
            mask_ranks: [
                MASK_RANK1_BB,
                MASK_RANK2_BB,
                MASK_RANK3_BB,
                MASK_RANK4_BB,
                MASK_RANK5_BB,
                MASK_RANK6_BB,
                MASK_RANK7_BB,
                MASK_RANK8_BB,
            ],
            clear_ranks: [
                CLEAR_RANK1_BB,
                CLEAR_RANK2_BB,
                CLEAR_RANK3_BB,
                CLEAR_RANK4_BB,
                CLEAR_RANK5_BB,
                CLEAR_RANK6_BB,
                CLEAR_RANK7_BB,
                CLEAR_RANK8_BB,
            ],
        }
    }

    pub const fn mask_rank(&self, rank: Rank) -> Bitboard {
        self.mask_ranks[rank as usize]
    }

    pub const fn clear_rank(&self, rank: Rank) -> Bitboard {
        self.clear_ranks[rank as usize]
    }
}

impl Index<(Rank, MaskOrClear)> for RankBBs {
    type Output = Bitboard;

    fn index(&self, index: (Rank, MaskOrClear)) -> &Self::Output {
        match index.1 {
            MaskOrClear::Mask => &self.mask_ranks[index.0 as usize],
            MaskOrClear::Clear => &self.clear_ranks[index.0 as usize],
        }
    }
}