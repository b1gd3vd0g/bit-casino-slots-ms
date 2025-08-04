//! This module defines the type alias `BinaryByte` which is used in the ByteBuilder machine to
//! store its value. It provides functionality for converting binary bytes to integer bytes, and
//! vice versa. It also provides a function to convert it into an easily readable bitstring.

pub type BinaryByte = [bool; 8];

/// Convert the binary byte **in place** to be its negative equivalent.
/// # Arguments
/// - `bin`: The binary byte
fn sign_change(bin: &mut BinaryByte) {
    for bit in bin.iter_mut() {
        *bit = !*bit;
    }
    for i in 7..0 {
        bin[i] = !bin[i];
        if !bin[i] {
            break;
        }
    }
}

/// Convert an integer into a binary byte.
/// # Arguments
/// - `dec`: The decimal number to convert
/// # Returns
/// The two's complement binary equivalent
#[allow(dead_code)]
pub fn to_binary(dec: i8) -> BinaryByte {
    let mut remaining = dec.clone();
    let base: i8 = 2;
    let neg = remaining < 0;
    remaining = (remaining as i16).abs() as i8;

    let mut bin = [false; 8];
    for (i, bit) in bin.iter_mut().enumerate() {
        let value = base.pow(7 - i as u32);
        if remaining - value >= 0 {
            remaining -= value;
            *bit = true;
        }
    }

    if neg {
        sign_change(&mut bin);
    }

    bin
}

/// Convert a binary byte into an integer.
/// # Arguments
/// - `bin`: The binary byte
/// # Returns
/// The equivalent integer
pub fn to_decimal(bin: &BinaryByte) -> i8 {
    let mut clone = bin.clone();
    let neg = clone[0];
    let base: i8 = 2;
    if neg {
        sign_change(&mut clone);
    }

    let mut dec = 0;
    for (i, bit) in clone.iter().enumerate() {
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

/// Convert a byte into a bitstring (usually for testing purposes).
/// # Arguments
/// - `bin` The binary byte
/// # Returns
/// The bitstring representing that byte
pub fn to_bitstring(bin: &BinaryByte) -> String {
    let mut bits = String::new();
    for (i, bit) in bin.iter().enumerate() {
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
