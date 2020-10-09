//! Module that for a piece - i.e. a part that is placed into the cube
//!
//! per piece there is a sequence of rotations, per dimensions described
//! that allows to address every single orientation of the piece in the 3d space
//! by an index.
//!
//! rotation map rotation index is mapped to a shape location
//! starting point is always (0,0,0)
//! e.g. half of one plain:
//!
//! ```ignore
//!   ++  +++ +++  ++
//! +++  ++     ++  +++
//! ```
//! There are 24 different rotations possible.
//! (8 rotations per plane/axis; 3 axis/planes)
//!
//! 2nd half is 90 degree rotated.
//!
//! Thus, the index of a piece denotes a very specific location of
//! a piece in space. It is a unique mapping. The index is composed
//! as unsigned 16 bit value
//!
//! ```ignore
//! +-------+-------+-------+-----------+-----+
//! | 0 1 2 | 3 4 5 | 6 7 8 | 9 A B C D | E F |
//! | x-offs| y-offs| z-offs| rotation  | 0 0 |
//! +-------+-------+-------+-----------+-----+
//! ```
//!
//! A positioned piece, determined by the index can fit into the
//! (potentially filled box) or not. It fits if all places are
//!
//! * inside the box
//! * the places are empty
use std::cmp::Ordering;

use super::cube;
use crate::cube::AddToBox;

const ROTATIONS: usize = 24;
const SHAPE_POINT: usize = 5;
const DIMENSIONS: usize = 3;

pub const PIECES: usize = 25;

const ROT_MAP: [[[isize; DIMENSIONS]; SHAPE_POINT]; ROTATIONS] = [
    // y - z plain
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 1, 2], [0, 1, 3]],
    [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 2], [0, 1, 3]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, -1, 2], [0, -1, 3]],
    [[0, 0, 0], [0, 0, 1], [0, -1, 1], [0, -1, 2], [0, -1, 3]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 2, 1], [0, 3, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 1, 1], [0, 2, 1], [0, 3, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 2, -1], [0, 3, -1]],
    [[0, 0, 0], [0, 1, 0], [0, 1, -1], [0, 2, -1], [0, 3, -1]],
    // x - z plain
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [1, 0, 2], [1, 0, 3]],
    [[0, 0, 0], [0, 0, 1], [1, 0, 1], [1, 0, 2], [1, 0, 3]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [-1, 0, 2], [-1, 0, 3]],
    [[0, 0, 0], [0, 0, 1], [-1, 0, 1], [-1, 0, 2], [-1, 0, 3]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 0, 1], [3, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [1, 0, 1], [2, 0, 1], [3, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 0, -1], [3, 0, -1]],
    [[0, 0, 0], [1, 0, 0], [1, 0, -1], [2, 0, -1], [3, 0, -1]],
    // x - y plain
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [1, 2, 0], [1, 3, 0]],
    [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 2, 0], [1, 3, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [-1, 2, 0], [-1, 3, 0]],
    [[0, 0, 0], [0, 1, 0], [-1, 1, 0], [-1, 2, 0], [-1, 3, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 1, 0], [3, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [1, 1, 0], [2, 1, 0], [3, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, -1, 0], [3, -1, 0]],
    [[0, 0, 0], [1, 0, 0], [1, -1, 0], [2, -1, 0], [3, -1, 0]],
];

const FIXED_MAP: [[isize; DIMENSIONS]; PIECES] = [
    // corners are 'a' to 'h'
    [0, 0, 0],
    [0, 4, 0],
    [0, 0, 4],
    [0, 4, 4],
    [4, 0, 0],
    [4, 4, 0],
    [4, 0, 4],
    [4, 4, 4],
    // inner corners are 'i' to 'p'
    [1, 1, 1],
    [1, 3, 1],
    [1, 1, 3],
    [1, 3, 3],
    [3, 1, 1],
    [3, 3, 1],
    [3, 1, 3],
    [3, 3, 3],
    // midpoints of sides are 'q' to 'v'
    [0, 2, 2],
    [2, 0, 2],
    [2, 2, 0],
    [4, 2, 2],
    [2, 4, 2],
    [2, 2, 4],
    // free pieces are 'w' to 'y' set to negative
    [-1, -1, -1],
    [-1, -1, -1],
    [-1, -1, -1],
];

#[derive(Clone, Debug)]
pub struct Piece {
    name_index: usize, // 0 .. 24
    x: isize,          // x index in [0..5]
    y: isize,          // y index in [0..5]
    z: isize,          // z index in [0..5]
    rotation: usize,   // rotation index in [0..ROTATIONS]
    // a piece is represented by  SHAPE_POINT number of points,
    // whereby each point is defined by 3-dim coordinates
    piece: [[isize; DIMENSIONS]; SHAPE_POINT],
}

impl Piece {
    pub fn new(name: usize) -> Piece {
        if name > 24 {
            panic!("Impossible piece name {}", name)
        };

        Piece {
            name_index: name,
            x: 0,
            y: 0,
            z: 0,
            rotation: 0, // zero element (in terms algebra)
            // mapped zero position  in 3-dim space
            piece: [[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 1, 0], [3, 1, 0]],
        }
    }
    /// Determine  the combination index from x,y,z and rotation
    ///
    /// ```ignore
    /// bit 0 .. 2 is for x
    /// bit 3 .. 5 is for y
    /// bit 6 .. 8 is for z
    /// bit 9 .. is for rotation
    /// ```
    pub fn get_combination(self: &Piece) -> u16 {
        let combination: u16 = (self.rotation as u16) << 3;
        // x,y, z must be > 0 in this case for valid combinations
        let combination: u16 = (combination | (self.z as u16)) << 3;
        let combination: u16 = (combination | (self.y as u16)) << 3;
        combination | (self.x as u16)
    }

    /// Set x,y,z and rotation according to the combination
    ///
    /// ```ignore
    /// bit 0 .. 2 is for x
    /// bit 3 .. 5 is for y
    /// bit 6 .. 8 is for z
    /// bit 9 .. is for rotation
    /// ```
    pub fn set_combination(self: &mut Piece, combination: Option<u16>) {
        // TODO check if use option parameter instead
        match combination {
            None => {
                self.x = 0;
                self.y = 0;
                self.z = 0;
                self.rotation = 0;
            }
            Some(combination) => {
                self.x = (combination & 0x7) as isize;
                let combination = combination >> 3;
                self.y = (combination & 0x7) as isize;
                let combination = combination >> 3;
                self.z = (combination & 0x7) as isize;
                self.rotation = (combination >> 3) as usize;
            }
        }
    }

    /// compute the next configuration of piece and assigns it
    ///
    /// returns true if it is the last configuration
    /// otherwise false
    pub fn next_config(self: &mut Piece) -> bool {
        self.rotation = self.rotation + 1;
        if self.rotation >= ROTATIONS {
            self.rotation = 0;
            self.x = self.x + 1;
        }
        if self.x >= cube::LENGTH as isize {
            self.x = 0;
            self.y = self.y + 1;
        }
        if self.y >= cube::WIDTH as isize {
            self.y = 0;
            self.z = self.z + 1;
        }
        self.z != cube::HEIGHT as isize
    }

    /// checks if a piece fits into a box
    ///
    /// returns true if it fits, otherwise false
    pub fn fit_in_box(self: &Piece) -> bool {
        for i in 0..SHAPE_POINT {
            if self.piece[i][0] < 0 || self.piece[i][0] >= cube::LENGTH as isize {
                return false;
            }
            if self.piece[i][1] < 0 || self.piece[i][1] >= cube::WIDTH as isize {
                return false;
            }
            if self.piece[i][2] < 0 || self.piece[i][2] >= cube::HEIGHT as isize {
                return false;
            }
        }
        true
    }

    /// Checks if a piece fits to its position that is bound to its index
    ///
    /// return true if it fits otherwise false
    pub fn fit_to_position(self: &Piece) -> bool {
        if self.name_index > 21 {
            // piece 22 w; 23 x and 24 y fit everywhere
            return true;
        }
        for i in 0..SHAPE_POINT {
            let fit = self.piece[i][0] == FIXED_MAP[self.name_index as usize][0];
            let fit = fit & (self.piece[i][1] == FIXED_MAP[self.name_index as usize][1]);
            let fit = fit & (self.piece[i][2] == FIXED_MAP[self.name_index as usize][2]);
            if fit {
                return true;
            }
        }
        false
    }

    pub fn set_piece(self: &mut Piece) -> &mut Piece {
        for i in 0..SHAPE_POINT {
            self.piece[i][0] = ROT_MAP[self.rotation as usize][i][0] + self.x;
            self.piece[i][1] = ROT_MAP[self.rotation as usize][i][1] + self.y;
            self.piece[i][2] = ROT_MAP[self.rotation as usize][i][2] + self.z;
        }
        self
    }

    /// Add a piece to a box
    ///
    ///
    pub fn add_to_box(self: &Piece, b: &mut cube::PrintBox) {
        for i in 0..SHAPE_POINT {
            b.add(
                self.piece[i][0] as usize,
                self.piece[i][1] as usize,
                self.piece[i][2] as usize,
                self.name_index,
            );
        }
    }

    /// Deliver the next possible location of a piece in the 5x5 cube
    ///
    /// start position all zero.
    /// Sequence is rotation, x, y
    ///
    pub fn is_config(self: &mut Piece) -> bool {
        self.set_piece();
        self.fit_in_box() && self.fit_to_position()
    }

    pub fn get_name(self: &Piece) -> usize {
        self.name_index
    }

    pub fn set_next_valid(self: &mut Piece, combination: u16) {
        self.set_combination(Some(combination));
        while self.is_config() {
            self.next_config();
        }
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Piece) -> Option<Ordering> {
        if self.name_index == other.name_index {
            return Some(Ordering::Equal);
        }
        if self.name_index < other.name_index {
            return Some(Ordering::Less);
        }
        Some(Ordering::Greater)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Piece) -> bool {
        self.name_index == other.name_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_piece_ok() {
        for name in 0..24 {
            let p = Piece::new(name);
            assert_eq!(p.x, 0);
            assert_eq!(p.y, 0);
            assert_eq!(p.z, 0);
            assert_eq!(p.rotation, 0);
            assert_eq!(p.name_index, name);
        }
    }

    #[test]
    #[should_panic]
    fn test_piece_create_panic_too_high() {
        Piece::new(25);
    }

    #[test]
    fn test_set_to_next_valid() {
        for name in 0..24 {
            let mut p = Piece::new(name);
            p.set_next_valid(1);
            assert!(p.x != 0 || p.y != 0 || p.z != 0 || p.rotation != 0);
        }
    }

    #[test]
    fn test_clone_it() {
        let p_orig = Piece::new(1);
        let mut p_clone = p_orig.clone();
        p_clone.piece[1][1] += 1;
        assert!(p_orig.x == p_clone.x);
        assert!(p_orig.piece[1][1] != p_clone.piece[1][1]);
        assert!(p_orig.piece[0][2] == p_clone.piece[0][2]);
    }
}
