![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
[![Docs](https://img.shields.io/badge/Docs-Passing-brightgreen)](https://docs.rs/barcode-rs/latest/barcode-rs/)
# barcode-rs

## Description

Barcode-rs is an utility crate for encoding data into several supported formats

Currently supported formats:
- Code11,
- EAN13,
- EAN8,
- Interleaved 2 of 5,
- Standard 2 of 5,

## Usage

```rust
let my_data = "7501031311309";
let result: bit_vec::BitVec = my_data.barcode_encode(BarcodeFormat::EAN13).unwrap();

```
```rust
let my_data = "7501031311309".chars().collect::<Vec<char>>();
let result: bit_vec::BitVec = barcode_rs::encode(&my_data, BarcodeFormat::EAN13).unwrap();

```


Current version: 0.1.0

License: MIT OR Apache-2.0
