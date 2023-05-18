use std::{array::TryFromSliceError, char::TryFromCharError};

use bit_vec::BitVec;
use format::{
    Code11, DynamicBarcodeEncoding, Interleaved2Of5, Standard2Of5, StaticBarcodeEncoding, EAN13,
    EAN8,
};

mod encoding_source;
pub mod format;

pub enum BarcodeFormat {
    Code11,
    EAN13,
    EAN8,
    Interleaved2Of5,
    Standard2Of5,
}

#[derive(Debug)]
pub enum EncodingError {
    WrongSize,
    WrongChar,
    WrongCheckusm,
}

impl From<TryFromSliceError> for EncodingError {
    fn from(_: TryFromSliceError) -> Self {
        Self::WrongSize
    }
}

impl From<TryFromCharError> for EncodingError {
    fn from(_: TryFromCharError) -> Self {
        Self::WrongChar
    }
}

pub fn encode(data: &[char], format: BarcodeFormat) -> Result<BitVec, EncodingError> {
    match format {
        BarcodeFormat::Code11 => Code11::encode(data),
        BarcodeFormat::EAN13 => {
            if data.len() == 12 {
                EAN13::encode(TryInto::<&[_; 12]>::try_into(data)?)
            } else if data.len() == 13 {
                EAN13::encode(TryInto::<&[_; 13]>::try_into(data)?)
            } else {
                Err(EncodingError::WrongSize)?
            }
        }
        BarcodeFormat::EAN8 => {
            if data.len() == 7 {
                EAN8::encode(TryInto::<&[_; 7]>::try_into(data)?)
            } else if data.len() == 8 {
                EAN8::encode(TryInto::<&[_; 8]>::try_into(data)?)
            } else {
                Err(EncodingError::WrongSize)?
            }
        }
        BarcodeFormat::Interleaved2Of5 => Interleaved2Of5::encode(data),
        BarcodeFormat::Standard2Of5 => Standard2Of5::encode(data),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::encoding_source::BorrowEncodingSource;

    use super::*;
    #[test]
    fn code_11() {
        let result = "123-4530".barcode_encode(BarcodeFormat::Code11).unwrap();
        let result = result
            .into_iter()
            .map(|a| if a { "1" } else { "0" })
            .collect::<String>();
        assert_eq!(
            &result,
            "10110010110101101001011011001010101101010110110110110101100101010101101011001"
        );
    }

    #[test]
    fn ean_13() {
        let result = "7501031311309"
            .barcode_encode(BarcodeFormat::EAN13)
            .unwrap();
        let result = result
            .into_iter()
            .map(|a| if a { "1" } else { "0" })
            .collect::<String>();

        assert_eq!(result, "10101100010100111001100101001110111101011001101010100001011001101100110100001011100101110100101".to_string());
    }
    #[test]
    fn ean_8() {
        let result = "55123457".barcode_encode(BarcodeFormat::EAN8).unwrap();
        let result = result
            .into_iter()
            .map(|a| if a { "1" } else { "0" })
            .collect::<String>();
        assert_eq!(
            result,
            "1010110001011000100110010010011010101000010101110010011101000100101".to_string()
        );
    }

    #[test]
    fn interleaved_2_of_5() {
        let result = "1234567"
            .barcode_encode(BarcodeFormat::Interleaved2Of5)
            .unwrap();
        let result2 = "12345670"
            .barcode_encode(BarcodeFormat::Interleaved2Of5)
            .unwrap();
        assert_eq!(result, result2);
        let result = result
            .into_iter()
            .map(|a| if a { "1" } else { "0" })
            .collect::<String>();
        assert_eq!(
            &result,
            "1010110100101011001101101001010011010011001010101010011001101101"
        );
    }
    #[test]
    fn standard_2_of_5() {
        let result = "12345670"
            .barcode_encode(BarcodeFormat::Standard2Of5)
            .unwrap();

        let result = result
            .into_iter()
            .map(|a| if a { "1" } else { "0" })
            .collect::<String>();

        assert_eq!(&result, "1101101011101010101110101110101011101110111010101010101110101110111010111010101011101110101010101011101110101011101110101101011");
    }
}
