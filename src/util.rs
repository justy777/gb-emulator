use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct DataUnit(usize);

impl DataUnit {
    #[must_use]
    pub const fn from_bytes(bytes: usize) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn to_bytes(self) -> usize {
        self.0
    }

    #[must_use]
    pub const fn to_kilobytes(self) -> usize {
        self.0 / 1024
    }

    #[must_use]
    pub const fn to_megabytes(self) -> usize {
        self.0 / 1024 / 1024
    }
}

impl Display for DataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.to_megabytes() > 0 {
            write!(f, "{} MiB", self.to_megabytes())
        } else if self.to_kilobytes() > 0 {
            write!(f, "{} KiB", self.to_kilobytes())
        } else {
            write!(f, "{} B", self.to_bytes())
        }
    }
}
