//! This module defines the ByteBuilder struct, the struct behind Bit Casino's first slot machine,
//! Byte Builder.
//!
//! # How it works:
//! - The screen has a single row with 8 columns. When spun, the machine generates either a `1` or a
//! `0` in each column.
//! - In order to spin, the player must wager 128 bits. The net gain/loss of the wager is equal to
//! the **two's compliment** equivalent of the represented byte.
//! - Any spin can have a multiplier [x1, x2, x4, x8, x16, x32, x64] which is applied to both the
//! wager and the winnings.

use rand::{Rng, rng};

use crate::machines::{
    SlotMachine,
    byte_builder::binary_byte::{BinaryByte, to_decimal},
};

pub mod binary_byte;

/// This is the struct storing all of the data for a ByteBuilder machine.
#[derive(Debug)]
pub struct ByteBuilder {
    /// The 8 "bits" representing the value used to determine the payout.
    byte: BinaryByte,
    /// The multiplier used to calculate payout.
    multiplier: u32,
    /// The payout for the currently displayed byte.
    payout: i128,
    /// The amount of spins that this machine has done.
    spins: u128,
    /// This machine's average payout.
    avg_payout: f64,
    /// This machine's running total payout
    total_payout: i128,
}

impl ByteBuilder {
    /// View the byte that is currently displayed.
    pub fn byte(&self) -> BinaryByte {
        self.byte
    }
}

impl SlotMachine for ByteBuilder {
    fn new() -> Self {
        ByteBuilder {
            byte: [false; 8],
            multiplier: 1,
            payout: 0,
            spins: 0,
            avg_payout: 0.0,
            total_payout: 0,
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
            *bit = rng.random_bool(0.5);
        }
        self.payout = to_decimal(&self.byte) as i128 * self.multiplier as i128;
        self.spins += 1;
        self.total_payout += self.payout;
        self.avg_payout = self.total_payout as f64 / self.spins as f64;
    }

    fn payout(&self) -> i128 {
        self.payout
    }
}
