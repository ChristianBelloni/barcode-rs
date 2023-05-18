use bit_vec::BitVec;

use crate::{format::dyn_calc_checksum, EncodingError};

use super::{DynamicBarcodeEncoding, Width, WIDTH_MAP};

/// Interleaved 2 of 5 format, for more [`info`](https://web.archive.org/web/20070202214904/http://barcodeisland.com/int2of5.phtml)
pub struct Interleaved2Of5;

impl DynamicBarcodeEncoding for Interleaved2Of5 {
    fn encode(chars: &[char]) -> Result<bit_vec::BitVec, crate::EncodingError> {
        if chars.len() % 2 != 0 {
            let mut v = chars.to_vec();
            let checksum = dyn_calc_checksum(&v);
            v.push(char::from_digit(checksum.into(), 10).unwrap());

            return Interleaved2Of5::encode(&v);
        }
        let mut ret = BitVec::new();
        ret.extend([true, false, true, false]);
        for c in chars.chunks(2) {
            let first = c[0];
            let second = c[1];
            let first_map = WIDTH_MAP.get(&first).ok_or(EncodingError::WrongChar)?;
            let second_map = WIDTH_MAP.get(&second).ok_or(EncodingError::WrongChar)?;
            for (is_bar, width) in EncodingIterator::new(first_map, second_map) {
                match width {
                    Width::Wide => ret.extend([is_bar, is_bar]),
                    Width::Narrow => ret.push(is_bar),
                }
            }
        }
        ret.extend([true, true, false, true]);
        Ok(ret)
    }
}

struct EncodingIterator<'a> {
    pub current: usize,
    pub first_chars: &'a [Width],
    pub second_chars: &'a [Width],
}

impl<'a> EncodingIterator<'a> {
    fn new(first_chars: &'a [Width], second_chars: &'a [Width]) -> Self {
        Self {
            current: 0,
            first_chars,
            second_chars,
        }
    }
}

impl<'a> Iterator for EncodingIterator<'a> {
    type Item = (bool, Width);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 10 {
            return None;
        }
        if self.current % 2 == 0 {
            let idx = self.current / 2;
            self.current += 1;
            Some((true, self.first_chars[idx]))
        } else {
            let idx = (self.current - 1) / 2;
            self.current += 1;
            Some((false, self.second_chars[idx]))
        }
    }
}
