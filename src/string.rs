use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::str::Chars;

/// Either a [`String`] or a static [`str`].
/// Hash code is cached and used for faster equivalence checks, though this makes the comparison a little less reliable.
#[derive(Clone, Eq, Debug)]
pub struct GewyString {
    data: StringData,
    hash: u64
}

impl Deref for GewyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl From<&'static str> for GewyString {
    fn from(value: &'static str) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Self {
            data: StringData::Static(value),
            hash: hasher.finish(),
        }
    }
}

impl From<String> for GewyString {
    fn from(value: String) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        Self {
            data: StringData::Owned(value),
            hash: hasher.finish(),
        }
    }
}

impl GewyString {
    pub fn chars(&self) -> Chars { self.data.chars() }
    pub fn hash(&self) -> u64 { self.hash }
}

impl PartialEq for GewyString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Ord for GewyString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl PartialOrd for GewyString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Hash for GewyString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Default for GewyString {
    fn default() -> Self {
        GewyString::from("")
    }
}

/// Raw data for a [`GewyString`].
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