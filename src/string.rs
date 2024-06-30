use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::str::Chars;

/// Either a [`String`] or a static [`str`].
/// Hash code is cached and used for faster equivalence checks, though this makes the comparison less reliable.
#[derive(Clone, Eq, Debug)]
pub struct UiString {
    data: StringData,
    hash: u64
}

impl Deref for UiString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl From<&'static str> for UiString {
    fn from(value: &'static str) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Self {
            data: StringData::Static(value),
            hash: hasher.finish(),
        }
    }
}

impl From<String> for UiString {
    fn from(value: String) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Self {
            data: StringData::Owned(value),
            hash: hasher.finish(),
        }
    }
}

impl UiString {
    pub fn chars(&self) -> Chars { self.data.chars() }
    pub fn hash(&self) -> u64 { self.hash }
}

impl PartialEq for UiString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Ord for UiString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl PartialOrd for UiString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Hash for UiString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Default for UiString {
    fn default() -> Self {
        UiString::from("")
    }
}

/// Any type that can be converted to a [`UiString`].
pub trait ToUiString {
    fn to_ui_string(self) -> UiString;
}

impl<S: Into<UiString>> ToUiString for S {
    fn to_ui_string(self) -> UiString {
        self.into()
    }
}

/// Raw string data for a [`UiString`].
#[derive(Clone, Eq, PartialEq, Debug)]
enum StringData {
    Static(&'static str),
    Owned(String),
}

impl Deref for StringData {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            StringData::Static(str) => str,
            StringData::Owned(string) => &string,
        }
    }
}

impl Ord for StringData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl PartialOrd for StringData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl Default for StringData {
    fn default() -> Self {
        StringData::Static("")
    }
}