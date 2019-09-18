use std::mem::size_of;

pub mod error;
pub mod prelude;
pub mod primitive;

pub use bytes;
pub use faster_hex;

// Little Endian
pub type Number = u32;
// Size of Number
pub const NUMBER_SIZE: usize = size_of::<Number>();
// Padding Character
pub const PADDING_CHAR: u8 = 0x00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alignment {
    Byte1 = 1,
    Byte2 = 2,
    Byte4 = 4,
    Byte8 = 8,
}

impl Alignment {
    #[inline]
    pub fn for_size(size: usize) -> Self {
        match size {
            1 => Alignment::Byte1,
            2 => Alignment::Byte2,
            3 | 4 => Alignment::Byte4,
            _ => Alignment::Byte8,
        }
    }

    #[inline]
    pub fn calc_padding(self, size: usize) -> usize {
        let alignment_size = self as usize;
        let extra_size = size % alignment_size;
        if extra_size == 0 {
            0
        } else {
            alignment_size - extra_size
        }
    }

    #[inline]
    pub fn calc_full_size(self, size: usize) -> usize {
        size + self.calc_padding(size)
    }
}

#[inline]
pub fn create_content(size: usize) -> Vec<u8> {
    Vec::with_capacity(size)
}

#[inline]
pub fn write_padding<W: ::std::io::Write>(writer: &mut W, size: usize) -> ::std::io::Result<()> {
    if size != 0 {
        writer.write_all(&vec![0u8; size])?;
    }
    Ok(())
}

#[inline]
pub fn unpack_number(slice: &[u8]) -> Number {
    #[allow(clippy::cast_ptr_alignment)]
    let le = slice.as_ptr() as *const Number;
    Number::from_le(unsafe { *le })
}

#[inline]
pub fn pack_number(num: Number) -> [u8; 4] {
    num.to_le_bytes()
}

#[inline]
pub fn unpack_number_vec(slice: &[u8]) -> &[[u8; 4]] {
    #[allow(clippy::cast_ptr_alignment)]
    unsafe {
        &*(slice as *const [u8] as *const [[u8; 4]])
    }
}

#[inline]
pub fn check_padding(slice: &[u8]) -> bool {
    slice.iter().all(|x| *x == PADDING_CHAR)
}
