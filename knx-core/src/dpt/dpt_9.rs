//! 3.10 Datapoint Types "2 Octet Float Value"

use std::fmt::Display;

use half::f16;
use snafu::{whatever, Whatever};

use crate::dpt::DptRaw;

/// 3.10 Datapoint Types "2 Octet Float Value"
/// DPT_Value_Temp
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DptValueTemp(pub f32);

impl DptRaw for DptValueTemp {
    type ByteArray = [u8; 2];

    fn id() -> (u8, u16) {
        (9, 1)
    }

    fn to_be_bytes(&self) -> Self::ByteArray {
        let sign = self.0 < 0.0;
        let v = self.0.abs();
        let v = v * 100.0;
        let m = v.floor();

        // Convert the decimal part to base 2
        //
        let mut f = v - m;
        let mut bits = 0u16;
        for i in 0..11 {
            f *= 2.0;
            if f >= 1.0 {
                bits |= 1 << (15 - i);
                f -= 1.0;
            }
        }

        let m = m as u32;
        // Combine real and decimal parts in a single u32
        // with decimal point at bit 16
        //
        let mantissa = m << 16 | bits as u32;

        let mut first_bit_set: i16 = 0;
        for i in (0..31).rev() {
            if (mantissa & 1 << i) > 0 {
                first_bit_set = i;
                break;
            }
        }
        // Exponent is how much we have to move the mantissa to have the first
        // bit inside the mantissa of the target f16 format
        //
        let exp = first_bit_set - 16 - 10;
        let mantissa: u32 = m << (31 - first_bit_set);

        let mut f16: u16 = 0;
        if sign {
            f16 = 0x8000;
        }
        f16 |= ((exp << 11) as u16) & 0b0111_1000_0000_0000;
        f16 |= (mantissa as u16 & 0b1111_1111_1110_0000) >> 5;
        f16.to_be_bytes()
    }

    fn from_be_bytes(bytes: Self::ByteArray) -> Result<Self, Whatever> {
        let value = u16::from_be_bytes(match bytes.try_into() {
            Ok(v) => v,
            Err(e) => whatever!("Wrong bytes count, {:?}", e),
        });
        let sign = if value & 0x8000 > 0 { -1f32 } else { 1f32 };
        let mut exp = ((value & 0b0111_1000_0000_0000) >> 11) as i16;
        if (exp & 8) > 0 {
            exp -= 1;
            exp ^= 0xf;
            exp += -1;
        }
        let mut mantissa = value & 0x07ff;
        if sign < 0.0 {
            mantissa -= 1;
            mantissa ^= 0x7ff;
        }
        let mantissa: f32 = mantissa as f32 * sign;
        Ok(Self((0.01 * mantissa) * (1 << exp) as f32))
    }
}

impl Display for DptValueTemp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C", self.0)
    }
}

// Implementation taken from java project calimero
// https://github.com/calimero-project/calimero-core/blob/d6fee46dbfeda9ddc90e77e25444dc1b1dd98f82/src/tuwien/auto/calimero/dptxlator/DPTXlator2ByteFloat.java#L300
//
fn calimero_float_to_bytes(v: f32) -> Vec<u8> {
    let mut v = v * 100.0;
    let mut exp = 0;
    while v < -2048.0 {
        v /= 2.0;
        exp += 1;
    }
    while v > 2048.0 {
        v /= 2.0;
        exp += 1;
    }

    let m = v.round() as i16 & 0x7ff;
    let mut msb: u8 = (exp << 3 | m >> 8) as u8;
    if v < 0.0 {
        msb |= 0x80;
    }
    let lsb = (m & 0xff) as u8;
    vec![msb, lsb]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        let test_values = vec![0.0, -10.8, -0.01, 188.95999];

        for value in test_values.into_iter() {
            let bytes = calimero_float_to_bytes(value);
            let parsed_value = DptValueTemp::from_be_bytes(bytes.try_into().unwrap()).expect("Should be able to parse value");
            assert_eq!(value, parsed_value.0);
        }
    }
}
