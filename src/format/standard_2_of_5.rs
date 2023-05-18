use bit_vec::BitVec;

use crate::{format::WIDTH_MAP, EncodingError};

use super::DynamicBarcodeEncoding;

/// Standard 2 of 5 format, for more [`info`](https://web.archive.org/web/20070202214456/http://barcodeisland.com/2of5.phtml)
pub struct Standard2Of5;

impl DynamicBarcodeEncoding for Standard2Of5 {
    fn encode(chars: &[char]) -> Result<bit_vec::BitVec, crate::EncodingError> {
        let mut ret = BitVec::new();
        ret.extend([true, true, false, true, true, false, true, false]);
        for ch in chars.iter() {
            let width_map = WIDTH_MAP.get(ch).ok_or(EncodingError::WrongChar)?;
            for i in width_map.iter() {
                match i {
                    crate::format::Width::Wide => ret.extend([true, true, true]),
                    crate::format::Width::Narrow => ret.push(true),
                }
                ret.push(false);
            }
        }
        ret.extend([true, true, false, true, false, true, true]);
        Ok(ret)
    }
}
