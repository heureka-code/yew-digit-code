use crate::{DigitCodeProfile, RefStrOrChar};

/// Predefined code profile for a TOTP code.
///
/// The length is given as a generic argument of type `usize` and defaults to 6.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TotpCodeProfile<const LENGTH: usize = 6>;

impl<const LENGTH: usize> TotpCodeProfile<LENGTH> {
    pub fn new() -> Self {
        Self {}
    }
}

impl<const LENGTH: usize> DigitCodeProfile for TotpCodeProfile<LENGTH> {
    fn len(&self) -> usize {
        LENGTH
    }
    fn char_matches_alphabet_impl(&self, char: RefStrOrChar) -> bool {
        "0123456789".contains(char)
    }
    fn input_mode(&self, _index: usize) -> &str {
        "numeric"
    }
}
