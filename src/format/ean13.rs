use std::println;

use bit_vec::BitVec;

use crate::EncodingError;

use super::{calc_checksum, Parity, ParityTable, StaticBarcodeEncoding};

pub struct EAN13;

impl StaticBarcodeEncoding<12> for EAN13 {
    fn encode(chars: &[char; 12]) -> Result<BitVec, EncodingError> {
        let first_char = chars[0];
        let second_char = chars[1];

        let table = ParityTable::new(first_char);

        let mut ret = BitVec::new();
        ret.extend([true, false, true]);
        ret.extend(Parity::Odd.encode(second_char));
        (2..7).for_each(|i| {
            ret.extend(table.left_hand_encode_for(chars[i], i - 2));
        });
        ret.extend([false, true, false, true, false]);
        (7..12).for_each(|i| ret.extend(ParityTable::right_hand_encode(chars[i])));
        let checked = calc_checksum(chars);
        ret.extend(ParityTable::right_hand_encode(
            char::from_digit(checked.into(), 10).unwrap(),
        ));
        ret.extend([true, false, true]);
        Ok(ret)
    }
}

impl StaticBarcodeEncoding<13> for EAN13 {
    fn encode(chars: &[char; 13]) -> Result<BitVec, EncodingError> {
        let first_char = chars[0];
        let second_char = chars[1];
        let last_char: u8 = chars[12].to_digit(10).unwrap() as _;

        let calc_checksum = calc_checksum::<12>(&chars[0..12].try_into()?);
        if last_char != calc_checksum {
            println!("calc: {calc_checksum} received: {last_char}");
            return Err(EncodingError::WrongCheckusm);
        }

        let table = ParityTable::new(first_char);

        let mut ret = BitVec::new();
        ret.extend([true, false, true]);
        ret.extend(Parity::Odd.encode(second_char));
        (2..7).for_each(|i| {
            ret.extend(table.left_hand_encode_for(chars[i], i - 2));
        });
        ret.extend([false, true, false, true, false]);
        (7..13).for_each(|i| ret.extend(ParityTable::right_hand_encode(chars[i])));
        ret.extend([true, false, true]);
        Ok(ret)
    }
}
