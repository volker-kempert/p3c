//! Module modules a cube 5x5x5
//!
//!

pub const LENGTH: usize = 5;
pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 5;
const MIN_VAL: isize = 0;
const MAX_VAL: isize = 24;

pub trait AddToBox {
    fn add(&mut self, x: usize, y: usize, z: usize, val: usize);
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrintBox {
    value: [[[isize; LENGTH]; WIDTH]; HEIGHT],
}

use std::fmt;

impl fmt::Display for PrintBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // something else here
        write!(f, "\n")?; // empty line
        for i in 0..LENGTH {
            for j in 0..WIDTH {
                for k in 0..HEIGHT {
                    // explicit way to demonstrate ? would solve
                    match write!(f, "{}", super::i2c(self.value[i][j][k])) {
                        Err(why) => return Err(why),
                        Ok(file) => file,
                    }
                }
                write!(f, " ")?;
            }
            write!(f, "\n")?; // empty line
        }
        write!(f, "\n")
    }
}

/// Add something to the bux to the box
///
/// at position (x,y,z) value val is set.
/// except there is already something at the position.
///
/// In this case MAX_VAL + 1 is set
///
impl AddToBox for PrintBox {
    fn add(&mut self, x: usize, y: usize, z: usize, val: usize) {
        if self.value[x][y][z] < MIN_VAL {
            self.value[x][y][z] = val as isize;
        } else {
            self.value[x][y][z] = MAX_VAL + 1;
        }
    }
}

impl PrintBox {
    pub fn new() -> PrintBox {
        PrintBox {
            value: [[[-1; LENGTH]; WIDTH]; HEIGHT],
        }
    }

    /// determine how many positions are occupied
    ///
    /// returns the a number in range of 0..125
    pub fn occupied_positions(self) -> usize {
        let mut count: usize = 0;
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    if self.value[x][y][z] > MIN_VAL && self.value[x][y][z] > MAX_VAL {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_print_box() {
        assert_eq!(
            PrintBox::new(),
            PrintBox {
                value: [
                    [
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1]
                    ],
                    [
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1]
                    ],
                    [
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1]
                    ],
                    [
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1]
                    ],
                    [
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1],
                        [-1, -1, -1, -1, -1]
                    ],
                ]
            }
        );
    }
}
