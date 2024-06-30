use std::collections::BTreeMap;
use std::path::Path;
use vello::peniko::{Blob, Font};
use crate::UiString;

/// A query for a particular [`Font`](crate::paint::Font).
/// Analagous to the various font-* properties in css.
#[derive(Clone, PartialEq, Debug)]
pub struct FontQuery {
    pub family: UiString,
    pub weight: u32,
    pub style: FontStyle,
    pub size: u32,
}

impl FontQuery {
    fn is_more_similar(
        &self,
        font_meta: &FontMeta,
        last_font_meta: &FontMeta,
    ) -> bool {
        if font_meta.family == self.family && last_font_meta.family != self.family {
            return true;
        }
        let font_weight_diff = font_meta.weight.abs_diff(self.weight);
        let last_font_weight_diff = last_font_meta.weight.abs_diff(self.weight);
        if font_weight_diff < last_font_weight_diff {
            return true;
        }
        if font_meta.style == self.style && last_font_meta.style != self.style {
            return true;
        }
        false
    }
}

impl Default for FontQuery {
    fn default() -> Self {
        Self {
            family: FONT_FAMILY_DEFAULT.into(),
            weight: FONT_WEIGHT_NORMAL,
            style: FontStyle::Normal,
            size: 16,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight
pub const FONT_FAMILY_DEFAULT: &'static str = "DEFAULT_FONT";
pub const FONT_WEIGHT_NORMAL: u32           = 400;
pub const FONT_WEIGHT_BOLD: u32             = 700;


/// A database font in-memory fonts to be queried for using a [`FontQuery`].
pub struct FontDB {
    default_font: Font,
    entries: BTreeMap<FontMeta, Font>,
}

impl FontDB {

    pub fn new(default_font: Font) -> Self {
        Self {
            default_font,
            entries: BTreeMap::new()
        }
    }

    pub fn load(default_font: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let font_bytes = std::fs::read(default_font.as_ref())?;
        let font_blob = Blob::from(font_bytes);
        Ok(Self {
            default_font: Font::new(font_blob, 0),
            entries: BTreeMap::new()
        })
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

    pub fn insert(&mut self, meta: FontMeta, value: Font) {
        self.entries.insert(meta, value);
    }

    /// Queries for the font that is most similar to the query.
    /// Falls back to default font if not found.
    pub fn query(&self, query: &FontQuery) -> &Font {
        if self.entries.is_empty() {
            return &self.default_font
        }
        let mut entries_iter = self.entries.iter();
        let mut selected_entry = entries_iter.next().unwrap();
        for (font_meta, font) in entries_iter {
            let last_font_meta = selected_entry.0;
            if query.is_more_similar(font_meta, last_font_meta) {
                selected_entry = (font_meta, font);
            }
        }
        selected_entry.1
    }
}


/// Metadata about a font in a [`FontDB`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct FontMeta {
    pub family: UiString,
    pub weight: u32,
    pub style: FontStyle,
}