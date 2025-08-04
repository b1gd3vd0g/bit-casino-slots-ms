//! This module handles the internal logic of all slot machines. Each speficic slot machine can be
//! found in its own sub-module here.\
//! This module describes the trait SlotMachine, which ALL slot machines must implement, regardless
//! of their complexity.

pub mod byte_builder;

/// This trait must be implemented by any slot machine struct defined within this module.
pub trait SlotMachine {
    /// Construct a new slot machine.
    fn new() -> Self;
    /// Set the multiplier on the slot machine.
    fn set_mult(&mut self, mult: u32);
    /// Spin the slot machine.
    fn spin(&mut self);
    /// Calculate the payout based on the **current state** of the slot machine.
    fn payout(&self) -> i128;
}
