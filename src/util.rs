/// Returns number of bits needed to represent n
pub const fn bits_needed(n: usize) -> usize {
    n.ilog2() as usize + 1
}

#[cfg(test)]
mod tests {
    use crate::util::bits_needed;

    #[test]
    fn test_bits_needed() {
        let n = bits_needed(32);
        assert_eq!(n, 6);
        let n = bits_needed(64);
        assert_eq!(n, 7);
    }
}
