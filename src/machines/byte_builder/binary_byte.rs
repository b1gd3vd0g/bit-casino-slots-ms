//! This module defines the type alias `BinaryByte` which is used in the ByteBuilder machine to
//! store its value. It provides functionality for converting binary bytes to integer bytes, and
//! vice versa. It also provides a function to convert it into an easily readable bitstring.

use std::fmt::{Debug, Display};

pub struct BinaryByte {
    bits: [bool; 8],
}

impl BinaryByte {
    /// Calculate the opposite BinaryByte.
    pub fn twos_comp(&self) -> Self {
        let mut bits = self.bits.clone();
        for bit in bits.iter_mut() {
            *bit = !*bit;
        }
        for i in 7..0 {
            bits[i] = !bits[i];
            if !bits[i] {
                break;
            }
        }
        Self { bits: bits }
    }

    pub fn to_bitstring(&self) -> String {
        let mut bits = String::new();
        for (i, bit) in self.bits.iter().enumerate() {
            bits.push(match *bit {
                true => '1',
                false => '0',
            });
            if i == 3 {
                bits.push(' ');
            }
        }
        bits
    }

    pub fn value(&self) -> i8 {
        let neg = self.bits[0];
        let base: i8 = 2;

        let bits = match neg {
            true => self.twos_comp().bits,
            false => self.bits,
        };

        let mut dec = 0;
        for (i, bit) in bits.iter().enumerate() {
            let value = (base as i16).pow(7 - i as u32) as i8;
            if *bit {
                dec += value;
            }
        }

        match neg {
            true => -1 - dec,
            false => dec,
        }
    }

    pub fn bits(&mut self) -> &mut [bool; 8] {
        &mut self.bits
    }

    pub fn new() -> Self {
        Self { bits: [false; 8] }
    }
}

impl Display for BinaryByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bits = self.to_bitstring();
        bits.push('\n');
        f.write_str(&bits)
    }
}

impl Debug for BinaryByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryByte")
            .field("bits", &self.to_bitstring())
            .finish()
    }
}
