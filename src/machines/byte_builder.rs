use rand::{Rng, rng};

use crate::machines::{
    SlotMachine,
    byte_builder::binary_byte::{BinaryByte, to_decimal},
};

pub mod binary_byte;

pub struct ByteBuilder {
    byte: BinaryByte,
    multiplier: u32,
}

impl ByteBuilder {
    pub fn byte(&self) -> BinaryByte {
        self.byte
    }
    pub fn mult(&self) -> u32 {
        self.multiplier
    }
}

impl SlotMachine for ByteBuilder {
    fn new() -> Self {
        ByteBuilder {
            byte: [false; 8],
            multiplier: 1,
        }
    }

    fn set_mult(&mut self, mult: u32) {
        self.multiplier = match mult {
            0 => 1,
            _ => mult,
        };
    }

    fn spin(&mut self) {
        let mut rng = rng();
        for bit in self.byte.iter_mut() {
            *bit = rng.random_bool(0.5)
        }
    }

    fn payout(&self) -> i128 {
        to_decimal(&self.byte) as i128 * self.multiplier as i128
    }
}
