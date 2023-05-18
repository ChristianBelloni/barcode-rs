use bit_vec::BitVec;

use crate::{
    format::{Parity, ParityTable},
    EncodingError,
};

use super::{calc_checksum, StaticBarcodeEncoding};

pub struct EAN8;

impl StaticBarcodeEncoding<7> for EAN8 {
    fn encode(chars: &[char; 7]) -> Result<bit_vec::BitVec, crate::EncodingError> {
        let checksum = calc_checksum::<7>(chars);
        let mut _chars = Vec::with_capacity(8);
        _chars.extend_from_slice(chars);
        _chars.push(char::from_digit(checksum as _, 10).unwrap());
        EAN8::encode(TryInto::<&[char; 8]>::try_into(_chars.as_slice())?)
    }
}

impl StaticBarcodeEncoding<8> for EAN8 {
    fn encode(chars: &[char; 8]) -> Result<bit_vec::BitVec, crate::EncodingError> {
        let last_char: u8 = chars[7].to_digit(10).unwrap() as _;

        let calc_checksum = calc_checksum::<7>(&chars[0..7].try_into()?);
        if last_char != calc_checksum {
            println!("calc: {calc_checksum} received: {last_char}");
            return Err(EncodingError::WrongCheckusm);
        }

        let mut ret = BitVec::new();
        ret.extend([true, false, true]);

        (0..4).for_each(|i| {
            ret.extend(Parity::Odd.encode(chars[i]));
        });

        ret.extend([false, true, false, true, false]);

        (4..8).for_each(|i| ret.extend(ParityTable::right_hand_encode(chars[i])));

        ret.extend([true, false, true]);
        Ok(ret)
    }
}
