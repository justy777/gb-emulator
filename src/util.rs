use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const BYTES_PER_KILO: usize = 1024;
const BYTES_PER_MEGA: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DataSize {
    bytes: usize,
}

impl DataSize {
    pub const ZERO: Self = Self::new(0);
    pub const MAX: Self = Self::new(usize::MAX);

    #[inline]
    #[must_use]
    pub const fn new(bytes: usize) -> Self {
        Self { bytes }
    }

    #[inline]
    #[must_use]
    pub const fn from_bytes(bytes: usize) -> Self {
        Self { bytes }
    }

    #[inline]
    #[must_use]
    pub const fn from_kilobytes(kilobytes: usize) -> Self {
        Self {
            bytes: kilobytes * BYTES_PER_KILO,
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_megabytes(megabytes: usize) -> Self {
        Self {
            bytes: megabytes * BYTES_PER_MEGA,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.bytes == 0
    }

    #[inline]
    #[must_use]
    pub const fn as_bytes(self) -> usize {
        self.bytes
    }

    #[inline]
    #[must_use]
    pub const fn as_kilobytes(self) -> usize {
        self.bytes / BYTES_PER_KILO
    }

    #[inline]
    #[must_use]
    pub const fn as_megabytes(self) -> usize {
        self.bytes / BYTES_PER_MEGA
    }

    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        if let Some(total) = self.bytes.checked_add(rhs.bytes) {
            Some(Self::new(total))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(result) => result,
            None => Self::MAX,
        }
    }

    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        if let Some(difference) = self.bytes.checked_sub(rhs.bytes) {
            Some(Self::new(difference))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(result) => result,
            None => Self::ZERO,
        }
    }

    #[inline]
    #[must_use]
    pub const fn checked_mul(self, rhs: u32) -> Option<Self> {
        if let Some(product) = self.bytes.checked_mul(rhs as usize) {
            Some(Self::new(product))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub const fn saturating_mul(self, rhs: u32) -> Self {
        match self.checked_mul(rhs) {
            Some(result) => result,
            None => Self::MAX,
        }
    }

    #[inline]
    #[must_use]
    pub const fn checked_div(self, rhs: u32) -> Option<Self> {
        if let Some(quotient) = self.bytes.checked_div(rhs as usize) {
            Some(Self::new(quotient))
        } else {
            None
        }
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = self.bytes;
        let mut suffix = "B";
        if value != 0 && value % BYTES_PER_MEGA == 0 {
            value /= BYTES_PER_MEGA;
            suffix = "MiB";
        } else if value != 0 && value % BYTES_PER_KILO == 0 {
            value /= BYTES_PER_KILO;
            suffix = "KiB";
        }

        write!(f, "{value} {suffix}")
    }
}

impl Add for DataSize {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs).expect("overflow when adding sizes")
    }
}

impl AddAssign for DataSize {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for DataSize {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("overflow when subtracting sizes")
    }
}

impl SubAssign for DataSize {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for DataSize {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self {
        self.checked_mul(rhs)
            .expect("overflow when multiplying size by scalar")
    }
}

impl Mul<DataSize> for u32 {
    type Output = DataSize;

    #[inline]
    fn mul(self, rhs: DataSize) -> DataSize {
        rhs * self
    }
}

impl MulAssign<u32> for DataSize {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for DataSize {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u32) -> Self {
        self.checked_div(rhs)
            .expect("divide by zero error when dividing size by scalar")
    }
}

impl DivAssign<u32> for DataSize {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Sum for DataSize {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| Self {
            bytes: a.bytes + b.bytes,
        })
    }
}

impl<'a> Sum<&'a Self> for DataSize {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| Self {
            bytes: a.bytes + b.bytes,
        })
    }
}

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
    use crate::util::{bit, bits_needed, DataSize};

    #[test]
    fn constructors() {
        assert_ne!(DataSize::from_bytes(1), DataSize::from_bytes(0));
        assert_eq!(DataSize::from_bytes(4), DataSize::new(4));
        assert_eq!(DataSize::from_bytes(0), DataSize::ZERO);

        assert_eq!(
            DataSize::from_megabytes(1),
            DataSize::from_bytes(1 * 1024 * 1024)
        );
        assert_eq!(DataSize::from_megabytes(0), DataSize::ZERO);

        assert_eq!(DataSize::from_kilobytes(1), DataSize::from_bytes(1 * 1024));
        assert_eq!(DataSize::from_kilobytes(0), DataSize::ZERO);
    }

    #[test]
    #[should_panic]
    fn from_kilobytes_overflow() {
        let _ = DataSize::from_kilobytes(usize::MAX);
    }

    #[test]
    #[should_panic]
    fn from_megabytes_overflow() {
        let _ = DataSize::from_megabytes(usize::MAX);
    }

    #[test]
    fn bytes() {
        assert_eq!(DataSize::new(0).as_bytes(), 0);
        assert_eq!(DataSize::new(7).as_bytes(), 7);
        assert_eq!(DataSize::from_bytes(1).as_bytes(), 1);
        assert_eq!(DataSize::from_kilobytes(999).as_bytes(), 999 * 1024);
        assert_eq!(DataSize::from_megabytes(999).as_bytes(), 999 * 1024 * 1024);
    }

    #[test]
    fn kilobytes() {
        assert_eq!(DataSize::new(0).as_kilobytes(), 0);
        assert_eq!(DataSize::new(2048).as_kilobytes(), 2);
        assert_eq!(DataSize::from_bytes(1023).as_kilobytes(), 0);
        assert_eq!(DataSize::from_bytes(1025).as_kilobytes(), 1);
        assert_eq!(DataSize::from_kilobytes(7).as_kilobytes(), 7);
        assert_eq!(DataSize::from_megabytes(2).as_kilobytes(), 2048);
    }

    #[test]
    fn megabytes() {
        assert_eq!(DataSize::new(0).as_megabytes(), 0);
        assert_eq!(DataSize::new(2097152).as_megabytes(), 2);
        assert_eq!(DataSize::from_bytes(1048575).as_megabytes(), 0);
        assert_eq!(DataSize::from_bytes(1048577).as_megabytes(), 1);
        assert_eq!(DataSize::from_kilobytes(1023).as_megabytes(), 0);
        assert_eq!(DataSize::from_kilobytes(1025).as_megabytes(), 1);
        assert_eq!(DataSize::from_megabytes(7).as_megabytes(), 7);
    }

    #[test]
    #[should_panic]
    fn overflow_add() {
        let _ = DataSize::MAX + DataSize::new(1);
    }

    #[test]
    fn add() {
        assert_eq!(DataSize::new(0) + DataSize::new(0), DataSize::new(0));
        assert_eq!(DataSize::new(1) + DataSize::new(2), DataSize::new(3));
    }

    #[test]
    fn checked_add() {
        assert_eq!(
            DataSize::new(0).checked_add(DataSize::new(1)),
            Some(DataSize::new(1))
        );
        assert_eq!(
            DataSize::new(1).checked_add(DataSize::new(2)),
            Some(DataSize::new(3))
        );
        assert_eq!(
            DataSize::new(1).checked_add(DataSize::new(usize::MAX)),
            None
        );
    }

    #[test]
    fn saturating_add() {
        assert_eq!(
            DataSize::new(0).saturating_add(DataSize::new(1)),
            DataSize::new(1)
        );
        assert_eq!(
            DataSize::new(1).saturating_add(DataSize::new(2)),
            DataSize::new(3)
        );
        assert_eq!(
            DataSize::new(1).saturating_add(DataSize::new(usize::MAX)),
            DataSize::MAX
        );
    }

    #[test]
    #[should_panic]
    fn overflow_sub() {
        let _ = DataSize::ZERO - DataSize::new(1);
    }

    #[test]
    fn sub() {
        assert_eq!(DataSize::new(1) - DataSize::new(1), DataSize::new(0));
        assert_eq!(DataSize::new(3) - DataSize::new(2), DataSize::new(1));
    }

    #[test]
    fn checked_sub() {
        assert_eq!(
            DataSize::new(1).checked_sub(DataSize::ZERO),
            Some(DataSize::new(1))
        );
        assert_eq!(
            DataSize::new(4).checked_sub(DataSize::new(2)),
            Some(DataSize::new(2))
        );
        assert_eq!(DataSize::ZERO.checked_sub(DataSize::new(1)), None);
    }

    #[test]
    fn saturating_sub() {
        assert_eq!(
            DataSize::new(1).saturating_sub(DataSize::ZERO),
            DataSize::new(1)
        );
        assert_eq!(
            DataSize::new(7).saturating_sub(DataSize::new(4)),
            DataSize::new(3)
        );
        assert_eq!(
            DataSize::ZERO.saturating_sub(DataSize::new(1)),
            DataSize::ZERO
        );
    }

    #[test]
    #[should_panic]
    fn overflow_mul() {
        let _ = DataSize::MAX * 2;
    }

    #[test]
    fn mul() {
        assert_eq!(DataSize::ZERO * 2, DataSize::ZERO);
        assert_eq!(DataSize::new(1) * 3, DataSize::new(3));
    }

    #[test]
    fn checked_mul() {
        assert_eq!(DataSize::ZERO.checked_mul(2), Some(DataSize::ZERO));
        assert_eq!(DataSize::new(3).checked_mul(5), Some(DataSize::new(15)));
        assert_eq!(DataSize::MAX.checked_mul(2), None);
    }

    #[test]
    fn saturating_mul() {
        assert_eq!(DataSize::ZERO.saturating_mul(3), DataSize::ZERO);
        assert_eq!(DataSize::new(2) * 3, DataSize::new(6));
        assert_eq!(DataSize::MAX.saturating_mul(2), DataSize::MAX);
    }

    #[test]
    fn div() {
        assert_eq!(DataSize::ZERO / 2, DataSize::ZERO);
        assert_eq!(DataSize::new(6) / 3, DataSize::new(2));
    }

    #[test]
    fn checked_div() {
        assert_eq!(DataSize::ZERO.checked_div(2), Some(DataSize::ZERO));
        assert_eq!(DataSize::new(10).checked_div(5), Some(DataSize::new(2)));
        assert_eq!(DataSize::new(3).checked_div(0), None);
    }

    #[test]
    fn sum() {
        let sizes = [DataSize::new(1), DataSize::new(2), DataSize::new(5)];
        let sum: DataSize = sizes.iter().sum();
        assert_eq!(sum, DataSize::new(8));
    }

    #[test]
    fn display_formatting_bytes() {
        assert_eq!(format!("{}", DataSize::new(7)), "7 B");
        assert_eq!(format!("{}", DataSize::new(88)), "88 B");
        assert_eq!(format!("{}", DataSize::new(999)), "999 B");
    }

    #[test]
    fn display_formatting_kilobytes() {
        assert_eq!(format!("{}", DataSize::from_kilobytes(7)), "7 KiB");
        assert_eq!(format!("{}", DataSize::from_kilobytes(88)), "88 KiB");
        assert_eq!(format!("{}", DataSize::from_kilobytes(999)), "999 KiB");
    }

    #[test]
    fn display_formatting_megabytes() {
        assert_eq!(format!("{}", DataSize::from_megabytes(7)), "7 MiB");
        assert_eq!(format!("{}", DataSize::from_megabytes(88)), "88 MiB");
        assert_eq!(format!("{}", DataSize::from_megabytes(999)), "999 MiB");
    }

    #[test]
    fn size_const() {
        const SIZE: DataSize = DataSize::new(1048576);

        const KILOBYTES: usize = SIZE.as_kilobytes();
        assert_eq!(KILOBYTES, 1024);

        const MEGABYTES: usize = SIZE.as_megabytes();
        assert_eq!(MEGABYTES, 1);

        const IS_ZERO: bool = DataSize::ZERO.is_zero();
        assert!(IS_ZERO);

        const FROM_BYTES: DataSize = DataSize::from_bytes(1);
        assert_eq!(FROM_BYTES, DataSize::new(1));

        const FROM_KILOBYTES: DataSize = DataSize::from_kilobytes(1);
        assert_eq!(FROM_KILOBYTES, DataSize::new(1024));

        const FROM_MEGABYTES: DataSize = DataSize::from_megabytes(1);
        assert_eq!(FROM_MEGABYTES, DataSize::new(1048576));

        const MAX: DataSize = DataSize::new(usize::MAX);

        const CHECKED_ADD: Option<DataSize> = MAX.checked_add(DataSize::new(1));
        assert_eq!(CHECKED_ADD, None);

        const CHECKED_SUB: Option<DataSize> = DataSize::ZERO.checked_sub(DataSize::new(1));
        assert_eq!(CHECKED_SUB, None);

        const CHECKED_MUL: Option<DataSize> = DataSize::new(1).checked_mul(1);
        assert_eq!(CHECKED_MUL, Some(DataSize::new(1)));

        const SATURATING_ADD: DataSize = MAX.saturating_add(DataSize::new(1));
        assert_eq!(SATURATING_ADD, MAX);

        const SATURATING_SUB: DataSize = DataSize::ZERO.saturating_sub(DataSize::new(1));
        assert_eq!(SATURATING_SUB, DataSize::ZERO);

        const SATURATING_MUL: DataSize = MAX.saturating_mul(2);
        assert_eq!(SATURATING_MUL, MAX);
    }

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
