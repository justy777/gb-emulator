/// Returns byte with bit n set, bits start from right at 0
pub const fn bit(n: usize) -> u8 {
    assert!(n < 8);
    1 << n
}

#[macro_export]
macro_rules! bits {
    ($( $n:expr),* ) => {
        {
            let mut val = 0;
            $(
                assert!($n < 8);
                val |= 1 << $n;
            )*
            val
        }
    };
}

/// Returns number of bits needed to represent n
pub const fn bits_needed(n: usize) -> usize {
    n.ilog2() as usize + 1
}

#[cfg(test)]
mod tests {
    use crate::util::{bit, bits_needed};

    #[test]
    fn test_bit() {
        assert_eq!(0b0000_0001, bit(0));
        assert_eq!(0b0000_0010, bit(1));
        assert_eq!(0b0000_0100, bit(2));
        assert_eq!(0b0000_1000, bit(3));
        assert_eq!(0b0001_0000, bit(4));
        assert_eq!(0b0010_0000, bit(5));
        assert_eq!(0b0100_0000, bit(6));
        assert_eq!(0b1000_0000, bit(7));
    }

    #[test]
    fn test_bits() {
        let n: u8 = bits!(0);
        assert_eq!(n, 0b0000_0001);
        let n: u8 = bits!(0, 1);
        assert_eq!(n, 0b0000_0011);
    }

    #[test]
    fn test_bits_needed() {
        let n = bits_needed(32);
        assert_eq!(n, 6);
        let n = bits_needed(64);
        assert_eq!(n, 7);
    }
}
