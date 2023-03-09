

struct Bitboards {
    all_bbs: [u64; 12],

    all_white_pieces_bb: u64, // all white pieces
    all_black_pieces_bb: u64, // all black pieces
    all_pieces_bb: u64   // all pieces
}

impl Bitboards {
    // updates the s
    fn update_bbs(&mut self, piece: &Piece, color: &Color, new_bitboard: u64) {
        self.all_bbs[Self::get_bb_index(piece, color)] = new_bitboard;

        match color {
            Color::White => {
                self.all_white_pieces_bb = 0;
                self.all_bbs[0..6].iter().map(|x| self.all_white_pieces_bb |= x);
            },
            Color::Black => {
                self.all_black_pieces_bb = 0;
                self.all_bbs[6..12].iter().map(|x| self.all_black_pieces_bb |= x);
            },
        }
        self.all_pieces_bb = self.all_white_pieces_bb | self.all_black_pieces_bb;
    }

    fn get_bb_index(piece: &Piece, color: &Color) -> usize {
        piece as usize + color.into() as usize
    }
}


#[repr(usize)]
enum Color {
    White = 0,
    Black = 6
}

#[repr(usize)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

enum File {
    A = 0x00000000000000FF,
    B = 0x000000000000FF00,
    C = 0x0000000000FF0000,
    D = 0x00000000FF000000,
    E = 0x000000FF00000000,
    F = 0x0000FF0000000000,
    G = 0x00FF000000000000,
    H = 0xFF00000000000000
}

enum Rank {
    First =     0x0101010101010101,
    Second
}

enum Diagonal {
    A1toH8 = 0x8040201008040201,
    H1toA8 = 0x0102040810204080
}

enum ColoredSquare {
    Light = 0x55AA55AA55AA55AA,
    Dark =  0xAA55AA55AA55AA55 
}