use crate::DigitCodeProfile;
#[cfg(feature = "itertools")]
use itertools::Itertools;
use std::sync::Arc;

#[cfg(feature = "unicode-segmentation")]
pub type ArcStrOrChar = Arc<str>;
#[cfg(not(feature = "unicode-segmentation"))]
pub type ArcStrOrChar = char;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DigitCode<PROFILE: DigitCodeProfile + 'static> {
    code: Vec<Option<ArcStrOrChar>>,
    profile: Arc<PROFILE>,
    update_indicator: i64,
}

impl<PROFILE: DigitCodeProfile + 'static> Default for DigitCode<PROFILE> {
    fn default() -> Self {
        Self::new(Arc::new(PROFILE::default()))
    }
}

impl<PROFILE: DigitCodeProfile + 'static> DigitCode<PROFILE> {
    pub fn new(profile: Arc<PROFILE>) -> Self {
        Self {
            code: vec![None; profile.len()],
            profile,
            update_indicator: 0,
        }
    }
    pub fn set(&mut self, index: usize, value: Option<ArcStrOrChar>) -> Option<()> {
        if index < self.code.len() {
            if let Some(value) = value.clone() {
                if !self.profile.is_valid_char(&value) {
                    return None;
                }
            }
            self.code[index] = value;
            Some(())
        } else {
            None
        }
    }
    pub fn profile(&self) -> Arc<PROFILE> {
        self.profile.clone()
    }
    pub fn len(&self) -> usize {
        self.profile.len()
    }
    pub fn get(&self, index: usize) -> &Option<ArcStrOrChar> {
        self.code.get(index).unwrap_or(&None)
    }
    pub fn clear(&mut self) {
        for val in self.code.iter_mut() {
            *val = None;
        }
    }
    pub fn as_empty(&self) -> Self {
        let mut val = self.clone();
        val.clear();
        val
    }
    #[allow(unused)]
    pub fn with_set(&self, index: usize, value: impl Into<Option<ArcStrOrChar>>) -> Self {
        let mut val = self.clone();
        val.set(index, value.into());
        val
    }
    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = &Option<ArcStrOrChar>> {
        self.code.iter()
    }
    pub fn iter_some(&self) -> impl Iterator<Item = &ArcStrOrChar> {
        self.code.iter().flatten()
    }

    pub fn joined(&self) -> Option<String> {
        if self.is_valid() {
            #[cfg(feature = "itertools")]
            let res = self.iter_some().join("");
            #[cfg(not(feature = "itertools"))]
            let res = {
                self.iter_some()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            };
            Some(res)
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        #[cfg(feature = "unicode-segmentation")]
        let chars = self.code.iter().flatten().map(|o| {
            let o: &str = o;
            o
        });
        #[cfg(not(feature = "unicode-segmentation"))]
        let chars = self.code.iter().flatten().cloned();
        self.profile.is_char_code_valid(chars)
    }

    pub(crate) fn change_update_indicator(&mut self) {
        self.update_indicator = (self.update_indicator + 1) % (i64::MAX - 10);
    }
}
