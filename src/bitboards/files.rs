use std::ops::Index;

use strum_macros::{EnumIter, AsRefStr};

use super::types::Bitboard;

#[derive(Copy, Clone, EnumIter, AsRefStr)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

// Masks
const MASK_FILE_A_BB: Bitboard = 0x101010101010101;
const MASK_FILE_B_BB: Bitboard = MASK_FILE_A_BB << 1;
const MASK_FILE_C_BB: Bitboard = MASK_FILE_B_BB << 1;
const MASK_FILE_D_BB: Bitboard = MASK_FILE_C_BB << 1;
const MASK_FILE_E_BB: Bitboard = MASK_FILE_D_BB << 1;
const MASK_FILE_F_BB: Bitboard = MASK_FILE_E_BB << 1;
const MASK_FILE_G_BB: Bitboard = MASK_FILE_F_BB << 1;
const MASK_FILE_H_BB: Bitboard = MASK_FILE_G_BB << 1;

// Clears
const CLEAR_FILE_A_BB: Bitboard = !MASK_FILE_A_BB;
const CLEAR_FILE_B_BB: Bitboard = !MASK_FILE_B_BB;
const CLEAR_FILE_C_BB: Bitboard = !MASK_FILE_C_BB;
const CLEAR_FILE_D_BB: Bitboard = !MASK_FILE_D_BB;
const CLEAR_FILE_E_BB: Bitboard = !MASK_FILE_E_BB;
const CLEAR_FILE_F_BB: Bitboard = !MASK_FILE_F_BB;
const CLEAR_FILE_G_BB: Bitboard = !MASK_FILE_G_BB;
const CLEAR_FILE_H_BB: Bitboard = !MASK_FILE_H_BB;

pub struct FileBBs {
    mask_files: [Bitboard; 8],
    clear_files: [Bitboard; 8],
}

impl FileBBs {
    pub const fn new() -> FileBBs {
        FileBBs {
            mask_files: [
                MASK_FILE_A_BB,
                MASK_FILE_B_BB,
                MASK_FILE_C_BB,
                MASK_FILE_D_BB,
                MASK_FILE_E_BB,
                MASK_FILE_F_BB,
                MASK_FILE_G_BB,
                MASK_FILE_H_BB,
            ],
            clear_files: [
                CLEAR_FILE_A_BB,
                CLEAR_FILE_B_BB,
                CLEAR_FILE_C_BB,
                CLEAR_FILE_D_BB,
                CLEAR_FILE_E_BB,
                CLEAR_FILE_F_BB,
                CLEAR_FILE_G_BB,
                CLEAR_FILE_H_BB,
            ],
        }
    }

    pub const fn mask_file(&self, file: File) -> Bitboard {
        self.mask_files[file as usize]
    }

    pub const fn clear_file(&self, file: File) -> Bitboard {
        self.clear_files[file as usize]
    }
}

impl Index<File> for FileBBs {
    type Output = Bitboard;

    fn index(&self, index: File) -> &Self::Output {
        &self.mask_files[index as usize]
    }
}