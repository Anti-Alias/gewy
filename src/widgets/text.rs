use taffy::Dimension;
use vello::glyph::skrifa::instance::Size;
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
            _font: None,
            _width: 0.0,
            _height: 0.0,
        }
    }
}

impl Text {
    fn compute_size(&mut self, max_width: f32) {
        let mut width = 0.0;
        let mut x = 0.0;
        let mut y = 0.0;
        let font = self._font.as_ref().unwrap();
        let font_query = &self.font;

        let line_height = self.line_height * self.font.size as f32;
        let variations: &[(&str, f32)] = &[];
        let font_ref = to_font_ref(font).unwrap();
        let axes = font_ref.axes();
        let font_size = Size::new(font_query.size as f32);
        let var_loc = axes.location(variations);
        let charmap = font_ref.charmap();
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        for c in self.string.chars() {
            if c == '\n' {
                y += line_height;
                x = 0.0;
                continue;
            }
            let gid = charmap.map(c).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
            x += advance;
            if x > max_width {
                x = 0.0;
                y += line_height;
            }
            if x > width {
                width = x;
            }
        }

        self._width = width;
        self._height = y;
    }

    fn size(&self) -> (f32, f32) {
        let width = self.width.unwrap_or(self._width);
        let height = self.height.unwrap_or(self._height);
        (width, height)
    }
}

impl Widget for Text {

    fn style(&self, style: &mut Style) {
        let (width, height) = self.size();
        style.size.width = Dimension::Length(width);
        style.size.height = Dimension::Length(height);
        style.max_size.width = Dimension::Length(width);
        style.max_size.height = Dimension::Length(height);
        style.min_size.width = Dimension::Length(width);
        style.min_size.height = Dimension::Length(height);
    }

    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {
        let width = self.width.unwrap_or(self._width);
        let affine = affine.then_translate(Vec2::new(layout.location.x as f64, layout.location.y as f64));

        let font_query = &self.font;
        let font = self._font.as_ref().unwrap();

        let variations: &[(&str, f32)] = &[];
        let line_height = self.line_height * self.font.size as f32;
        let font_ref = to_font_ref(font).unwrap();
        let axes = font_ref.axes();
        let font_size = Size::new(font_query.size as f32);
        let var_loc = axes.location(variations);
        let charmap = font_ref.charmap();
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        let mut pen_x = 0f32;
        let mut pen_y = 0f32;
        scene
            .draw_glyphs(font)
            .brush(&Brush::Solid(self.color))
            .font_size(font_query.size as f32)
            .transform(affine)
            .draw(Fill::NonZero, self.string.chars().filter_map(|c| {
                if c == '\n' {
                    pen_y += line_height;
                    pen_x = 0.0;
                    return None;
                }
                let gid = charmap.map(c).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                let x = pen_x;
                let y = pen_y;
                pen_x += advance;
                if pen_x > width {
                    pen_y += line_height;
                    pen_x = 0.0;
                }
                Some(Glyph { id: gid.to_u16() as u32, x, y })
            }));
    }
}

pub fn text(string: impl Into<GewyString>, class: impl Class<Text>, r: &mut UIRenderer) -> NodeId {
    let mut text = Text { string: string.into(), ..Default::default() };
    class.apply(&mut text);
    let font = r.font_db().query(&text.font).clone();
    text._font = Some(font);
    if text.width.is_none() || text.height.is_none() {
        let max_width = text.width.unwrap_or(f32::MAX);
        text.compute_size(max_width);
    }
    r.insert(text)
}

fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    use vello::skrifa::raw::FileRef;
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}