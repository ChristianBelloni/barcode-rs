mod code_11;
mod common_2_of_5_char_set;
mod ean13;
mod ean8;
mod ean_char_set;
mod interleaved_2_of_5;
mod standard_2_of_5;

pub(crate) use common_2_of_5_char_set::*;
pub(crate) use ean_char_set::*;

use crate::EncodingError;
use bit_vec::BitVec;

pub trait StaticBarcodeEncoding<const T: usize>: private::Sealed {
    fn encode(chars: &[char; T]) -> Result<BitVec, EncodingError>;
}

pub trait DynamicBarcodeEncoding: private::Sealed {
    fn encode(chars: &[char]) -> Result<BitVec, EncodingError>;
}

mod private {
    use super::*;
    pub trait Sealed {}
    impl Sealed for Code11 {}
    impl Sealed for EAN13 {}
    impl Sealed for EAN8 {}
    impl Sealed for Interleaved2Of5 {}
    impl Sealed for Standard2Of5 {}
}

pub use code_11::Code11;
pub use ean13::EAN13;
pub use ean8::EAN8;
pub use interleaved_2_of_5::Interleaved2Of5;
pub use standard_2_of_5::Standard2Of5;
