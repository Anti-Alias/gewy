use taffy::{AvailableSpace, Dimension};
use vello::glyph::skrifa::charmap::Charmap;
use vello::glyph::skrifa::instance::Size as FontSize;
use vello::glyph::skrifa::metrics::GlyphMetrics;
use vello::glyph::skrifa::{FontRef, MetadataProvider};
use vello::glyph::Glyph;
use vello::kurbo::{Rect, RoundedRect, RoundedRectRadii, Vec2};
use vello::peniko::{Brush, Fill};

use crate::{Class, FontQuery, GewyString, WidgetId, Scene, UIRenderer, Widget};
use crate::geom::Affine;
use crate::layout::{Style, Layout, Size};
use crate::paint::{Font, Color};

/// A simple text [`Widget`](crate::Widget).
pub struct Text {
    pub string: GewyString,
    pub font: FontQuery,
    pub line_height: f32,
    pub color: Color,
    pub background_color: Color,
    pub size: Size<Dimension>,
    pub text_align: TextAlign,
    pub word_wrap: bool,
    _glyphs: Vec<Glyph>,    // Glyphs computed at measure time
    _font: Option<Font>,    // Computed font
}

impl Default for Text {
    fn default() -> Self {
        Self {
            string: "".into(),
            font: FontQuery::default(),
            line_height: 1.2,
            color: Color::WHITE,
            background_color: Color::TRANSPARENT,
            size: Size::auto(),
            text_align: TextAlign::default(),
            word_wrap: true,
            _glyphs: vec![],
            _font: None,
        }
    }
}

impl Widget for Text {

    fn measure(&mut self, known_size: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        match (known_size.width, available_space.width) {
            (None, AvailableSpace::Definite(def_width)) => {
                let mut lines = GlyphLines::new(
                    &self.string,
                    self._font.as_ref().unwrap(),
                    self.line_height,
                    self.font.size,
                    def_width,
                    self.word_wrap,
                );
                lines.align(self.text_align, def_width);
                self._glyphs = lines.glyphs;
                Size { width: lines.width, height: lines.height }
            },
            (None, AvailableSpace::MinContent) => {
                let lines = GlyphLines::new(
                    &self.string,
                    self._font.as_ref().unwrap(),
                    self.line_height,
                    self.font.size,
                    0.0,
                    self.word_wrap,
                );
                self._glyphs = lines.glyphs;
                Size { width: 0.0, height: lines.height }
            },
            (None, AvailableSpace::MaxContent) => {
                let lines = GlyphLines::new(
                    &self.string,
                    self._font.as_ref().unwrap(),
                    self.line_height,
                    self.font.size,
                    f32::INFINITY,
                    self.word_wrap,
                );
                self._glyphs = lines.glyphs;
                Size { width: lines.width, height: lines.height }
            },
            (Some(known_width),_) => {
                let mut lines = GlyphLines::new(
                    &self.string,
                    self._font.as_ref().unwrap(),
                    self.line_height,
                    self.font.size,
                    known_width,
                    self.word_wrap,
                );
                lines.align(self.text_align, 10000.0);
                self._glyphs = lines.glyphs;
                self._glyphs[0].x += 200.0;
                Size { width: known_width, height: lines.height }
            },
        }
    }

    fn style(&self, style: &mut Style) {
        style.size = self.size;
        style.min_size = self.size;
        style.max_size = self.size;
    }

    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {
        
        // Paints background
        if self.background_color.a != 0  {
            let rect = Rect::new(
                layout.location.x as f64,
                layout.location.y as f64,
                (layout.location.x + layout.size.width) as f64,
                (layout.location.y + layout.size.height) as f64,
            );
            scene.fill(Fill::NonZero, affine, self.background_color, None, &rect);
        }

        // Paints text
        let font = self._font.as_ref().unwrap();
        let affine = affine.then_translate(Vec2::new(layout.location.x as f64, layout.location.y as f64));
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
) -> WidgetId {
    // Configures text
    let mut text = Text { string: string.into(), ..Default::default() };
    class.apply(&mut text);
    // Finalizes text
    let font = renderer.font_db().query(&text.font).clone();
    text._font = Some(font);
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
        let font_size = FontSize::new(font_size);
        let var_loc = axes.location(variations);
        let charmap = font_ref.charmap();
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        let metrics = font_ref.metrics(font_size, &var_loc);
        let offset_y = metrics.ascent - metrics.descent + metrics.leading;

        if !word_wrap {
            Self::non_wrapping(string, offset_y, line_height, max_width, &glyph_metrics, &charmap)
        }
        else {
            Self::non_wrapping(string, offset_y, line_height, max_width, &glyph_metrics, &charmap)
        }
    }

    fn align(&mut self, align: TextAlign, width: f32) {
        match align {
            TextAlign::Left => {},
            TextAlign::Right => self.align_right(width),
            TextAlign::Center => self.align_center(width),
        };
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
        offset_y: f32,
        max_width: f32,
        glyph_metrics: &GlyphMetrics,
        charmap: &Charmap
    ) -> Self {

        let mut glyphs = vec![];
        let mut line_metas = vec![];
        let mut pen_x: f32 = 0.0;
        let mut pen_y: f32 = offset_y;
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let mut line_meta = LineMeta { index: 0, width: 0.0 };
        let mut tokens = string.split(" ").filter(|token| !token.is_empty());

        // Logic for consuming a character
        let mut consume_char = |c: char, line_meta: &mut LineMeta, line_metas: &mut Vec<LineMeta>| {
            let glyph_id = charmap.map(c).unwrap_or_default();
            let glyph_advance = glyph_metrics.advance_width(glyph_id).unwrap_or_default();
            if c == '\n' || pen_x + glyph_advance > max_width {
                line_meta.width = pen_x;
                line_metas.push(*line_meta);
                *line_meta = LineMeta { index: glyphs.len(), width: 0.0 };
                pen_x = 0.0;
                pen_y += line_height;
                if c == '\n' { return };
            }
            glyphs.push(Glyph {
                id: glyph_id.to_u16() as u32,
                x: pen_x,
                y: pen_y
            });
            width = width.max(pen_x + glyph_advance);
            height = height.max(pen_y + line_height);
            pen_x += glyph_advance;
        };

        // Consumes all characters from all tokens.
        // Places space between each.
        let mut next_token = tokens.next();
        while let Some(token) = next_token {
            for c in token.chars() {
                consume_char(c, &mut line_meta, &mut line_metas);
            }
            next_token = tokens.next();
            if next_token.is_some() {
                consume_char(' ', &mut line_meta, &mut line_metas);
            }
        }
        // Finishes
        line_meta.width = pen_x;
        line_metas.push(line_meta);
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

#[derive(Copy, Clone, Default, Debug)]
struct LineMeta {
    index: usize,
    width: f32,
}

#[derive(Default, Debug)]
struct WordMeta {
    index: usize,
    width: f32,
}