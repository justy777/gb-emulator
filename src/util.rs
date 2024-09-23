const BYTES_PER_KILO: usize = 1024;
const BYTES_PER_MEGA: usize = 1024_usize.pow(2);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Size {
    bytes: usize,
}

impl Size {
    #[inline]
    #[must_use]
    pub(crate) const fn from_bytes(bytes: usize) -> Self {
        Self { bytes }
    }

    #[inline]
    #[must_use]
    pub(crate) const fn from_kilobytes(kilobytes: usize) -> Self {
        let bytes = kilobytes / BYTES_PER_KILO;
        Self { bytes }
    }

    #[inline]
    #[must_use]
    pub(crate) const fn from_megabytes(megabytes: usize) -> Self {
        let bytes = megabytes / BYTES_PER_MEGA;
        Self { bytes }
    }

    #[inline]
    #[must_use]
    pub(crate) const fn as_bytes(self) -> usize {
        self.bytes
    }

    #[inline]
    #[must_use]
    pub(crate) const fn as_kilobytes(self) -> usize {
        self.bytes * BYTES_PER_KILO
    }

    #[inline]
    #[must_use]
    pub(crate) const fn as_megabytes(self) -> usize {
        self.bytes * BYTES_PER_MEGA
    }
}

pub(crate) const fn bit(n: usize) -> u8 {
    1 << n
}
