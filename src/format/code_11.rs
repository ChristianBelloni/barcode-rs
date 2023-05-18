use bit_vec::BitVec;

use crate::EncodingError;

use super::DynamicBarcodeEncoding;

pub struct Code11;

impl DynamicBarcodeEncoding for Code11 {
    fn encode(chars: &[char]) -> Result<BitVec, EncodingError> {
        let mut ret = BitVec::new();
        ret.extend([true, false, true, true, false, false, true, false]);
        for c in chars {
            match c {
                '0' => ret.extend([true, false, true, false, true, true, false]),
                '1' => ret.extend([true, true, false, true, false, true, true, false]),
                '2' => ret.extend([true, false, false, true, false, true, true, false]),
                '3' => ret.extend([true, true, false, false, true, false, true, false]),
                '4' => ret.extend([true, false, true, true, false, true, true, false]),
                '5' => ret.extend([true, true, false, true, true, false, true, false]),
                '6' => ret.extend([true, false, false, true, true, false, true, false]),
                '7' => ret.extend([true, false, true, false, false, true, true, false]),
                '8' => ret.extend([true, true, false, true, false, false, true, false]),
                '9' => ret.extend([true, true, false, true, false, true, false]),
                '-' => ret.extend([true, false, true, true, false, true, false]),
                _ => continue,
            };
        }
        ret.extend([true, false, true, true, false, false, true]);
        Ok(ret)
    }
}
