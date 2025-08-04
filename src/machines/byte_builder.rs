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
    curr_payout: i128,
    /// The amount of spins that this machine has done (NOT including free spins).
    total_spins: u128,
    /// This machine's average payout.
    avg_payout: f64,
    /// This machine's running total payout.
    total_payout: i128,
    /// The amount of free spins this machine has given.
    total_free_spins: u128,
    /// The amount of null byte events.
    total_null_bytes: u128,
    /// The amount of remaining free spins available.
    remaining_free_spins: u32,
}

impl ByteBuilder {
    /// View the byte that is currently displayed.
    pub fn byte(&self) -> BinaryByte {
        self.byte
    }

    fn determine_payout(&mut self) {
        // find out if any special events are happening.
        let mut first = 0;
        if self
            .byte
            .iter()
            .enumerate()
            .filter(|&(i, &b)| {
                if b && first == 0 {
                    first = i;
                }
                b
            })
            .count()
            == 1
        {
            // Power of Two event:
            self.remaining_free_spins += 1; // 8 - first as u32;
        }
        let byte = to_decimal(&self.byte);
        if byte == 0 {
            // Null Byte event:
            self.total_null_bytes += 1;
            println!("NULL BYTE EVENT");
            self.curr_payout = 256 * self.multiplier as i128;
        } else {
            self.curr_payout = byte as i128 * self.multiplier as i128;
        }
    }
}

impl SlotMachine for ByteBuilder {
    fn new() -> Self {
        ByteBuilder {
            byte: [false; 8],
            multiplier: 1,
            curr_payout: 0,
            avg_payout: 0.0,
            total_payout: 0,
            total_spins: 0,
            total_free_spins: 0,
            total_null_bytes: 0,
            remaining_free_spins: 0,
        }
    }

    fn set_mult(&mut self, mult: u32) {
        self.multiplier = match mult {
            0 => 1,
            _ => mult,
        };
    }

    fn spin(&mut self) {
        if self.remaining_free_spins == 0 {
            let mut rng = rng();
            for bit in self.byte.iter_mut() {
                *bit = rng.random_bool(0.65);
            }
            self.determine_payout();
            self.total_spins += 1;
            self.total_payout += self.curr_payout;
            self.avg_payout = self.total_payout as f64 / self.total_spins as f64;
        } else {
            self.free_spin();
        }
    }

    fn free_spin(&mut self) {
        let mut rng = rng();
        for (i, bit) in self.byte.iter_mut().enumerate() {
            *bit = match i {
                0 => false,
                _ => rng.random_bool(0.5),
            };
        }
        self.determine_payout();
        self.total_free_spins += 1;
        self.total_payout += self.curr_payout;
        self.avg_payout =
            self.total_payout as f64 / (self.total_spins + self.total_free_spins) as f64;
        self.remaining_free_spins -= 1;
    }

    fn payout(&self) -> i128 {
        self.curr_payout
    }
}
