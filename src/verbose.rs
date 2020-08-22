//! Manage verbosity in CLI
//! 

extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
enum Verbosity {
    QUIET = 0,
    SPARSE = 1,
    NORMAL = 2,
    NONSTOP = 3
}

let mut verbosity: Verbosity = QUIET;

pub set_verbosity(v: u8) {
    verbosity =  num::FromPrimitive::from_u8(v);
}

macro_rules! verbose_sparse {
    
}