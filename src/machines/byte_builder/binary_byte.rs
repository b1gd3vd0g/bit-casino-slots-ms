pub type BinaryByte = [bool; 8];

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

pub fn to_decimal(bin: &BinaryByte) -> i8 {
    let mut clone = bin.clone();
    let neg = clone[0];
    let base: i8 = 2;
    if neg {
        sign_change(&mut clone);
    }

    let mut dec = 0;
    for (i, bit) in clone.iter().enumerate() {
        let value = base.pow(7 - i as u32) as i8;
        if *bit {
            dec += value;
        }
    }

    match neg {
        true => 0 - dec,
        false => dec,
    }
}
