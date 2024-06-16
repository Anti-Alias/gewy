use std::str::Chars;

use taffy::Dimension;
use vello::glyph::skrifa::charmap::Charmap;
use vello::glyph::skrifa::instance::Size;
use vello::glyph::skrifa::metrics::GlyphMetrics;
use vello::glyph::skrifa::{FontRef, MetadataProvider};
use vello::glyph::Glyph;
use vello::kurbo::Vec2;
use vello::peniko::{Brush, Fill};

use crate::{Class, FontQuery, GewyString, NodeId, Scene, UIRenderer, Widget};
use crate::geom::Affine;
use crate::layout::{Style, Layout};
use crate::paint::{Font, Color};

/// A simple text [`Widget`](crate::Widget).
pub struct Text {
    pub string: GewyString,
    pub font: FontQuery,
    pub line_height: f32,
    pub color: Color,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub text_align: TextAlign,
    pub word_wrap: bool,
    _glyphs: Vec<Glyph>,
    _font: Option<Font>,        // Computed font
    _width: f32,                // Computed width
    _height: f32,               // Computed height
}

impl Default for Text {
    fn default() -> Self {
        Self {
            string: "".into(),
            font: FontQuery::default(),
            line_height: 1.2,
            color: Color::WHITE,
            width: None,
            height: None,
            text_align: TextAlign::default(),
            word_wrap: true,
            _glyphs: vec![],
            _font: None,
            _width: 0.0,
            _height: 0.0,
        }
    }
}

impl Text {

    fn finalize(&mut self, font: Font) {
        let mut lines = GlyphLines::new(
            &self.string,
            &font,
            self.line_height,
            self.font.size,
            self.width.unwrap_or(f32::MAX),
            self.word_wrap,
        );
        let width = self.width.unwrap_or(lines.width);
        let height = self.height.unwrap_or(lines.height);
        match self.text_align {
            TextAlign::Left => {},
            TextAlign::Right => lines.align_right(width),
            TextAlign::Center => lines.align_center(width),
        }
        self._glyphs = lines.glyphs;
        self._width = width;
        self._height = height;
        self._font = Some(font);
    }
}

impl Widget for Text {

    fn style(&self, style: &mut Style) {
        style.size.width = Dimension::Length(self._width);
        style.size.height = Dimension::Length(self._height);
        style.max_size.width = Dimension::Length(self._width);
        style.max_size.height = Dimension::Length(self._height);
        style.min_size.width = Dimension::Length(self._width);
        style.min_size.height = Dimension::Length(self._height);
    }

    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {
        let affine = affine.then_translate(Vec2::new(layout.location.x as f64, layout.location.y as f64));
        let font = self._font.as_ref().unwrap();
        scene
            .draw_glyphs(font)
            .brush(&Brush::Solid(self.color))
            .font_size(self.font.size as f32)
            .transform(affine)
            .draw(Fill::NonZero, self._glyphs.iter().copied());
    }
}

/// Alignment of text in a [`Text`] element.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
}

pub fn text(
    string: impl Into<GewyString>,
    class: impl Class<Text>,
    renderer: &mut UIRenderer
) -> NodeId {
    // Configures text
    let mut text = Text { string: string.into(), ..Default::default() };
    class.apply(&mut text);
    // Finalizes text
    let font = renderer.font_db().query(&text.font).clone();
    text.finalize(font);
    // Inserts text
    renderer.insert(text)
}

fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    use vello::skrifa::raw::FileRef;
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}

#[derive(Default, Debug)]
struct LineMeta {
    index: usize,
    width: f32,
}

#[derive(Debug)]
struct GlyphLines {
    glyphs: Vec<Glyph>,
    line_metas: Vec<LineMeta>,
    width: f32,
    height: f32,
}

impl GlyphLines {

    fn new(
        string: &str,
        font: &Font,
        line_height: f32,
        font_size: u32,
        max_width: f32,
        word_wrap: bool,
    ) -> Self {
        let font_size = font_size as f32;
        let variations: &[(&str, f32)] = &[];
        let line_height = line_height * font_size;
        let font_ref = to_font_ref(font).unwrap();
        let axes = font_ref.axes();
        let font_size = Size::new(font_size);
        let var_loc = axes.location(variations);
        let charmap = font_ref.charmap();
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        if !word_wrap {
            Self::non_wrapping(string, line_height, max_width, &glyph_metrics, &charmap)
        }
        else {
            Self::non_wrapping(string, line_height, max_width, &glyph_metrics, &charmap)
        }
    }

    fn align_right(&mut self, width: f32) {
        for i in 0..self.line_metas.len() {
            let line_meta = &self.line_metas[i];
            let offset = width - line_meta.width;
            let line_start = line_meta.index;
            let line_end = &self.line_metas.get(i+1).map(|meta| meta.index);
            let line_end = line_end.unwrap_or(self.glyphs.len());
            for glyph_idx in line_start..line_end {
                let glyph = &mut self.glyphs[glyph_idx];
                glyph.x += offset;
            }
        }
    }

    fn align_center(&mut self, width: f32) {
        let center = width / 2.0;
        for i in 0..self.line_metas.len() {
            let line_meta = &self.line_metas[i];
            let line_center = line_meta.width / 2.0;
            let offset = center - line_center;
            let line_start = line_meta.index;
            let line_end = &self.line_metas.get(i+1).map(|meta| meta.index);
            let line_end = line_end.unwrap_or(self.glyphs.len());
            for glyph_idx in line_start..line_end {
                let glyph = &mut self.glyphs[glyph_idx];
                glyph.x += offset;
            }
        }
    }

    fn non_wrapping(
        string: &str,
        line_height: f32,
        max_width: f32,
        glyph_metrics: &GlyphMetrics,
        charmap: &Charmap
    ) -> Self {

        let mut glyphs = vec![];
        let mut line_metas = vec![];
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut height: f32 = 0.0;
        let mut line_meta = LineMeta { index: 0, width: 0.0 };

        for (index, c) in string.chars().enumerate() {

            // Get glyph, and move down a line if it overflows
            let gid = charmap.map(c).unwrap_or_default();
            let gid_advance = glyph_metrics.advance_width(gid).unwrap_or_default();
            if c == '\n' || x + gid_advance > max_width {
                line_meta.width = x;
                line_metas.push(line_meta);
                line_meta = LineMeta { index, width: 0.0 };
                x = 0.0;
                y += line_height;
                if c == '\n' { continue };
            }

            // Add glyph and advance cursor
            glyphs.push(Glyph { id: gid.to_u16() as u32, x, y });
            height = height.max(y + line_height);
            x += gid_advance;
        }

        line_meta.width = x;
        line_metas.push(line_meta);

        let mut width: f32 = 0.0;
        for line_width in line_metas.iter().map(|meta| meta.width) {
            width = width.max(line_width);
        }
        Self { glyphs, line_metas, width, height }
    }

    fn wrapping(
        string: &str,
        line_height: f32,
        max_width: f32,
        glyph_metrics: &GlyphMetrics,
        charmap: &Charmap
    ) -> Self {
        todo!()
    }
}
