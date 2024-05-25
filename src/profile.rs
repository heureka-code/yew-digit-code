use std::fmt::Debug;

#[cfg(feature = "itertools")]
use itertools::Itertools;

#[cfg(feature = "unicode-segmentation")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "unicode-segmentation")]
pub type RefStrOrChar<'a> = &'a str;
#[cfg(not(feature = "unicode-segmentation"))]
pub type RefStrOrChar<'a> = char;

#[cfg(not(feature = "unicode-segmentation"))]
pub(self) mod char_and_ref {
    pub trait CharOrCharRef {
        fn as_char(&self) -> char;
    }
    impl CharOrCharRef for char {
        fn as_char(&self) -> char {
            self.clone()
        }
    }
    impl CharOrCharRef for &char {
        fn as_char(&self) -> char {
            **self
        }
    }
}
#[cfg(not(feature = "unicode-segmentation"))]
use char_and_ref::CharOrCharRef;

/// This trait is implemented for structs that represent a code with a fixed amount of digits and alphabet.
///
/// IF YOUR ALPHABET CONTAINS UNICODE CODEPOINTS WHICH ARE LONGER THAN ONE SCALAR YOU HAVE TO ENABLE THE
/// `unicode-segmentation` feature
#[allow(clippy::len_without_is_empty)]
pub trait DigitCodeProfile: PartialEq + Clone + Default + Debug {
    /// This describes the amount of digits
    fn len(&self) -> usize;
    /// This needs to check if a given char matches the alphabet. Length is checked in other functions.
    /// Char will have length 1
    fn char_matches_alphabet_impl(&self, char: RefStrOrChar<'_>) -> bool;

    /// Checks if a provided text represents a valid character for a single digit
    ///
    /// if unicode-segmentation feature is disabled length will be checked with chars which could
    /// result in wrong result unless you only use unicode chars with length of one char.
    #[cfg(feature = "unicode-segmentation")]
    fn is_valid_char(&self, chr: RefStrOrChar<'_>) -> bool {
        chr.graphemes(true).count() == 1 && self.char_matches_alphabet_impl(chr)
    }
    #[cfg(not(feature = "unicode-segmentation"))]
    fn is_valid_char<'a, C: CharOrCharRef>(&self, chr: C) -> bool {
        self.char_matches_alphabet_impl(chr.as_char())
    }
    /// This methods should return the html input mode: text, numeric, ... that the digit on position `index` should have.
    /// Index will be in range 0 <= index < len()
    #[allow(unused_variables)]
    fn input_mode(&self, index: usize) -> &str {
        "text"
    }

    /// This function takes an iterator over text and checks
    ///
    /// 1. if every item is a valid char according to `is_valid_char`
    /// 2. if the iterator has the correct length (number of digits)
    fn is_char_code_valid<'a, I: Iterator<Item = RefStrOrChar<'a>>>(&'a self, chars: I) -> bool {
        let mut len = 0;
        for char in chars {
            len += 1;
            if !self.is_valid_char(char) {
                return false;
            }
        }
        len == self.len()
    }
    /// Checks if a provided text represents a valid digit code of this profile
    ///
    /// This splits the text in chars which will be done differently if unicode-segmentation is disabled.
    ///
    /// If you use chars in your alphabet that are longer than one unicode codepoint it could
    /// result in wrong behaviour.
    fn is_str_code_valid(&self, code: &str) -> bool {
        #[cfg(feature = "unicode-segmentation")]
        let v = code.graphemes(true);
        #[cfg(not(feature = "unicode-segmentation"))]
        let v = code.chars();
        self.is_char_code_valid(v.into_iter())
    }

    /// This checks if the provided code is valid.
    /// The user is responsible for splitting a text into chars/str
    fn valid_char_code(&self, chars: &[RefStrOrChar<'_>]) -> Option<String> {
        if self.is_char_code_valid(chars.iter().cloned()) {
            #[cfg(feature = "itertools")]
            let res = chars.into_iter().join("");
            #[cfg(not(feature = "itertools"))]
            let res = {
                chars
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            };
            Some(res)
        } else {
            None
        }
    }
}
