use core::borrow::{Borrow, BorrowMut};
use core::mem::{size_of, transmute};
use valida_derive::AlignedBorrow;
use valida_machine::Word;
use valida_util::indices_arr;

#[derive(AlignedBorrow, Default, Debug)]
pub struct Bitwise32Cols<T> {
    pub input_1: Word<T>,
    pub input_2: Word<T>,

    /// Bit decomposition of input_1 bytes
    pub bits_1: [[T; 8]; 4],

    /// Bit decomposition of input_2 bytes
    pub bits_2: [[T; 8]; 4],

    /// Aggregated output
    pub output: Word<T>,

    pub is_and: T,
    pub is_or: T,
    pub is_xor: T,
}

pub const NUM_BITWISE_COLS: usize = size_of::<Bitwise32Cols<u8>>();
pub const COL_MAP: Bitwise32Cols<usize> = make_col_map();

const fn make_col_map() -> Bitwise32Cols<usize> {
    let indices_arr = indices_arr::<NUM_BITWISE_COLS>();
    unsafe { transmute::<[usize; NUM_BITWISE_COLS], Bitwise32Cols<usize>>(indices_arr) }
}
