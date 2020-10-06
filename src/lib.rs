pub mod cube;
pub mod ge_cube;
pub mod piece;

/// i2c (index to char)
/// converts an index to a char for printing
///
/// invalid indexes are mapped to '.'
/// indexes between 0 and 24 are valid
///
pub fn i2c(index: isize) -> char {
    if index < 0 {
        return '.';
    }
    if index > 24 {
        return '#';
    }
    // i8 must be first safely converted to u8
    let index = index as u8;
    // note the ascii value for 'a' is 97
    (index + 97) as char
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_i2c() {
        // bad cases
        assert_eq!(i2c(-1), '.');
        assert_eq!(i2c(25), '#');

        // good cases
        assert_eq!(i2c(0), 'a');
        assert_eq!(i2c(7), 'h');
        assert_eq!(i2c(15), 'p');
        assert_eq!(i2c(24), 'y');
    }
}
