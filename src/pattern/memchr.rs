use core::{cmp, mem, usize};

#[allow(unused_imports)] use crate::shim::*;

const LO_U64: u64 = 0x0101010101010101;
const HI_U64: u64 = 0x8080808080808080;

const LO_USIZE: usize = LO_U64 as usize;
const HI_USIZE: usize = HI_U64 as usize;
const USIZE_BYTES: usize = mem::size_of::<usize>();

fn contains_zero_byte(x: usize) -> bool {
    x.wrapping_sub(LO_USIZE) & !x & HI_USIZE != 0
}

#[cfg(target_pointer_width = "16")]
fn repeat_byte(b: u8) -> usize {
    (b as usize) << 8 | b as usize
}

#[cfg(not(target_pointer_width = "16"))]
fn repeat_byte(b: u8) -> usize {
    (b as usize) * (usize::MAX / 255)
}

pub(super) fn memchr(x: u8, text: &[u8]) -> Option<usize> {
    if text.len() < 2 * USIZE_BYTES {
        return text.iter().position(|elt| *elt == x);
    }

    memchr_general_case(x, text)
}

fn memchr_general_case(x: u8, text: &[u8]) -> Option<usize> {
    let len = text.len();
    let ptr = text.as_ptr();
    let mut offset = ptr.align_offset(USIZE_BYTES);

    if offset > 0 {
        offset = cmp::min(offset, len);
        if let Some(index) = text[..offset].iter().position(|elt| *elt == x) {
            return Some(index);
        }
    }

    let repeated_x = repeat_byte(x);
    while offset <= len - 2 * USIZE_BYTES {
        unsafe {
            let u = *(ptr.add(offset) as *const usize);
            let v = *(ptr.add(offset + USIZE_BYTES) as *const usize);

            let zu = contains_zero_byte(u ^ repeated_x);
            let zv = contains_zero_byte(v ^ repeated_x);
            if zu || zv {
                break;
            }
        }
        offset += USIZE_BYTES * 2;
    }

    text[offset..].iter().position(|elt| *elt == x).map(|i| offset + i)
}

pub(super) fn memrchr(x: u8, text: &[u8]) -> Option<usize> {
    let len = text.len();
    let ptr = text.as_ptr();
    type Chunk = usize;

    let (min_aligned_offset, max_aligned_offset) = {
        let (prefix, _, suffix) = unsafe { text.align_to::<(Chunk, Chunk)>() };
        (prefix.len(), len - suffix.len())
    };

    let mut offset = max_aligned_offset;
    if let Some(index) = text[offset..].iter().rposition(|elt| *elt == x) {
        return Some(offset + index);
    }

    let repeated_x = repeat_byte(x);
    let chunk_bytes = mem::size_of::<Chunk>();

    while offset > min_aligned_offset {
        unsafe {
            let u = *(ptr.offset(offset as isize - 2 * chunk_bytes as isize) as *const Chunk);
            let v = *(ptr.offset(offset as isize - chunk_bytes as isize) as *const Chunk);

            let zu = contains_zero_byte(u ^ repeated_x);
            let zv = contains_zero_byte(v ^ repeated_x);
            if zu || zv {
                break;
            }
        }
        offset -= 2 * chunk_bytes;
    }

    text[..offset].iter().rposition(|elt| *elt == x)
}
