use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct TryFromUintError(pub(crate) ());

impl Display for TryFromUintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "out of range unsigned integer conversion attempted".fmt(f)
    }
}

impl Error for TryFromUintError {}
