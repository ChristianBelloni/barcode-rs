use bit_vec::BitVec;

pub fn calc_checksum<const T: usize>(chars: &[char; T]) -> u8 {
    let parsed: [u8; T] = chars.map(|a| a.to_digit(10).unwrap() as _);
    let mut tot = 0;
    for (i, val) in parsed.iter().rev().enumerate() {
        if i % 2 != 0 {
            tot += *val;
        } else {
            tot += *val * 3;
        }
    }
    tot %= 10;
    tot = 10 - tot;
    if tot == 10 {
        tot = 0;
    }
    tot
}

pub fn dyn_calc_checksum(chars: &[char]) -> u8 {
    let parsed: Vec<u8> = chars.iter().map(|a| a.to_digit(10).unwrap() as _).collect();
    let mut tot = 0;
    for (i, val) in parsed.iter().rev().enumerate() {
        if i % 2 != 0 {
            tot += *val;
        } else {
            tot += *val * 3;
        }
    }
    tot %= 10;
    tot = 10 - tot;
    if tot == 10 {
        tot = 0;
    }
    tot
}

pub struct ParityTable {
    inner: [Parity; 5],
}

impl ParityTable {
    pub fn left_hand_encode_for(&self, val: char, pos: usize) -> BitVec {
        let parity = self.inner[pos];
        parity.encode(val)
    }

    pub fn right_hand_encode(t: char) -> BitVec {
        match t {
            '0' => BitVec::from_iter([true, true, true, false, false, true, false]),
            '1' => BitVec::from_iter([true, true, false, false, true, true, false]),
            '2' => BitVec::from_iter([true, true, false, true, true, false, false]),
            '3' => BitVec::from_iter([true, false, false, false, false, true, false]),
            '4' => BitVec::from_iter([true, false, true, true, true, false, false]),
            '5' => BitVec::from_iter([true, false, false, true, true, true, false]),
            '6' => BitVec::from_iter([true, false, true, false, false, false, false]),
            '7' => BitVec::from_iter([true, false, false, false, true, false, false]),
            '8' => BitVec::from_iter([true, false, false, true, false, false, false]),
            '9' => BitVec::from_iter([true, true, true, false, true, false, false]),
            _ => panic!("char : {t}"),
        }
    }
}

impl ParityTable {
    pub fn new(first: char) -> ParityTable {
        match first {
            '0' => Self {
                inner: [Parity::Odd; 5],
            },
            '1' => Self {
                inner: [
                    Parity::Odd,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Even,
                ],
            },
            '2' => Self {
                inner: [
                    Parity::Odd,
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                ],
            },
            '3' => Self {
                inner: [
                    Parity::Odd,
                    Parity::Even,
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                ],
            },
            '4' => Self {
                inner: [
                    Parity::Even,
                    Parity::Odd,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Even,
                ],
            },
            '5' => Self {
                inner: [
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Odd,
                ],
            },
            '6' => Self {
                inner: [
                    Parity::Even,
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Odd,
                ],
            },
            '7' => Self {
                inner: [
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                ],
            },
            '8' => Self {
                inner: [
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                ],
            },
            '9' => Self {
                inner: [
                    Parity::Even,
                    Parity::Even,
                    Parity::Odd,
                    Parity::Even,
                    Parity::Odd,
                ],
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Parity {
    Odd,
    Even,
}

impl Parity {
    pub fn encode(&self, t: char) -> BitVec {
        match self {
            Parity::Odd => Self::encode_odd(t),
            Parity::Even => Self::encode_even(t),
        }
    }

    fn encode_even(t: char) -> BitVec {
        match t {
            '0' => BitVec::from_iter([false, true, false, false, true, true, true]),
            '1' => BitVec::from_iter([false, true, true, false, false, true, true]),
            '2' => BitVec::from_iter([false, false, true, true, false, true, true]),
            '3' => BitVec::from_iter([false, true, false, false, false, false, true]),
            '4' => BitVec::from_iter([false, false, true, true, true, false, true]),
            '5' => BitVec::from_iter([false, true, true, true, false, false, true]),
            '6' => BitVec::from_iter([false, false, false, false, true, false, true]),
            '7' => BitVec::from_iter([false, false, true, false, false, false, true]),
            '8' => BitVec::from_iter([false, false, false, true, false, false, true]),
            '9' => BitVec::from_iter([false, false, true, false, true, true, true]),
            _ => panic!(),
        }
    }
    fn encode_odd(t: char) -> BitVec {
        match t {
            '0' => BitVec::from_iter([false, false, false, true, true, false, true]),
            '1' => BitVec::from_iter([false, false, true, true, false, false, true]),
            '2' => BitVec::from_iter([false, false, true, false, false, true, true]),
            '3' => BitVec::from_iter([false, true, true, true, true, false, true]),
            '4' => BitVec::from_iter([false, true, false, false, false, true, true]),
            '5' => BitVec::from_iter([false, true, true, false, false, false, true]),
            '6' => BitVec::from_iter([false, true, false, true, true, true, true]),
            '7' => BitVec::from_iter([false, true, true, true, false, true, true]),
            '8' => BitVec::from_iter([false, true, true, false, true, true, true]),
            '9' => BitVec::from_iter([false, false, false, true, false, true, true]),
            _ => panic!(),
        }
    }
}
