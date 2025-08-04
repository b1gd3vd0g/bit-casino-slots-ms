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

pub fn to_binary(mut dec: i8) -> BinaryByte {
    let base: i8 = 2;
    let neg = dec < 0;
    dec = (dec as i16).abs() as i8;

    let mut bin = [false; 8];
    for (i, bit) in bin.iter_mut().enumerate() {
        let value = base.pow(7 - i as u32);
        if dec - value >= 0 {
            dec -= value;
            *bit = true;
        }
    }

    if neg {
        sign_change(&mut bin);
    }

    bin
}

pub fn to_decimal(mut bin: BinaryByte) -> i8 {
    let neg = bin[0];
    let base: i8 = 2;
    if neg {
        sign_change(&mut bin);
    }

    let mut dec = 0;
    for (i, bit) in bin.iter().enumerate() {
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
