use bit_vec::BitVec;

use crate::{encode, BarcodeFormat, EncodingError};

pub trait BorrowEncodingSource {
    fn barcode_encode(&self, format: BarcodeFormat) -> Result<BitVec, EncodingError>;
}

pub trait OwnedEncodingSource {
    fn barcode_encode_owned(self, format: BarcodeFormat) -> Result<BitVec, EncodingError>;
}

impl BorrowEncodingSource for String {
    fn barcode_encode(&self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        let data = self.chars().collect::<Vec<_>>();
        encode(&data, format)
    }
}

impl OwnedEncodingSource for String {
    fn barcode_encode_owned(mut self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        let data = self.drain(0..self.len()).collect::<Vec<_>>();
        encode(&data, format)
    }
}

impl BorrowEncodingSource for [char] {
    fn barcode_encode(&self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        encode(self, format)
    }
}

impl OwnedEncodingSource for &[char] {
    fn barcode_encode_owned(self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        encode(self, format)
    }
}

impl OwnedEncodingSource for &str {
    fn barcode_encode_owned(self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        self.to_string().barcode_encode_owned(format)
    }
}

impl BorrowEncodingSource for &str {
    fn barcode_encode(&self, format: BarcodeFormat) -> Result<BitVec, EncodingError> {
        self.to_string().barcode_encode(format)
    }
}
