use ethnum::{u256, U256};

pub fn bits_to_target(bits: u32) -> u256 {
    // Target = coefficient * 2 ^ ( 8 * (index — 3) )
    let exponent = bits >> 24;
    let coefficient = U256::new((bits & 0x00ffffff).into());

    coefficient << ((exponent - 3) * 8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bits_to_target_test() {
        // genesis block bits & difficulty
        let target_hex = "00000000ffff0000000000000000000000000000000000000000000000000000";
        let target = U256::from_str_radix(target_hex, 16).unwrap();

        assert_eq!(target, bits_to_target(486604799));

        // inspired from https://learnmeabitcoin.com/technical/bits
        let target_hex = "00000000000000000696f4000000000000000000000000000000000000000000";
        let target = U256::from_str_radix(target_hex, 16).unwrap();

        assert_eq!(target, bits_to_target(0x180696f4));
    }
}
