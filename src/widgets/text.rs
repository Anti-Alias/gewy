use taffy::{AvailableSpace, Dimension};
use vello::glyph::skrifa::charmap::Charmap;
use vello::glyph::skrifa::instance::Size as FontSize;
use vello::glyph::skrifa::metrics::GlyphMetrics;
use vello::glyph::skrifa::{FontRef, MetadataProvider};
use vello::glyph::Glyph;
use vello::kurbo::{Rect, Vec2};
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
                lines.align(self.text_align, lines.width);
                self._glyphs = lines.glyphs;
                Size { width: lines.width, height: lines.height }
            },
            (None, AvailableSpace::MinContent | AvailableSpace::MaxContent) => {
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
            (Some(known_width), _) => {
                let mut lines = GlyphLines::new(
                    &self.string,
                    self._font.as_ref().unwrap(),
                    self.line_height,
                    self.font.size,
                    known_width,
                    self.word_wrap,
                );
                lines.align(self.text_align, known_width);
                self._glyphs = lines.glyphs;
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


#[derive(Debug, Default)]
struct GlyphLines {
    glyphs: Vec<Glyph>,
    lines: Vec<Line>,
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
            Self::non_wrapping(string, offset_y, line_height, max_width, glyph_metrics, charmap)
        }
        else {
            Self::wrapping(string, offset_y, line_height, max_width, glyph_metrics, charmap)
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
        for i in 0..self.lines.len() {
            let line_meta = &self.lines[i];
            let offset = width - line_meta.width;
            let line_start = line_meta.index;
            let line_end = &self.lines.get(i+1).map(|meta| meta.index);
            let line_end = line_end.unwrap_or(self.glyphs.len());
            for glyph_idx in line_start..line_end {
                let glyph = &mut self.glyphs[glyph_idx];
                glyph.x += offset;
            }
        }
    }

    fn align_center(&mut self, width: f32) {
        let center = width / 2.0;
        for i in 0..self.lines.len() {
            let line_meta = &self.lines[i];
            let line_center = line_meta.width / 2.0;
            let offset = center - line_center;
            let line_start = line_meta.index;
            let line_end = &self.lines.get(i+1).map(|meta| meta.index);
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
        glyph_metrics: GlyphMetrics,
        charmap: Charmap
    ) -> Self {

        let mut result = Self::default();
        let mut current_line = Line::default();
        let mut pen_x: f32 = 0.0;
        let mut pen_y: f32 = offset_y;

        // Logic for consuming a single character into the result
        let mut consume_char = |c: char| {
            let glyph_id = charmap.map(c).unwrap_or_default();
            let glyph_advance = glyph_metrics.advance_width(glyph_id).unwrap_or_default();
            if c == '\n' || pen_x + glyph_advance > max_width {
                result.lines.push(current_line);
                current_line = Line { index: result.glyphs.len(), width: 0.0 };
                pen_x = 0.0;
                pen_y += line_height;
                if c == '\n' { return };
            }
            result.glyphs.push(Glyph {
                id: glyph_id.to_u16() as u32,
                x: pen_x,
                y: pen_y,
            });
            result.width = result.width.max(pen_x + glyph_advance);
            result.height = result.height.max(pen_y + line_height);
            current_line.width += glyph_advance;
            pen_x += glyph_advance;
        };

        // Tokenizes text, then consumes all characters.
        // Note: Mutliple whitespace characters will converted to single whitespace.
        let mut tokens = string.split(' ').filter(|token| !token.is_empty());
        let mut next_token = tokens.next();
        while let Some(token) = next_token {
            for c in token.chars() {
                consume_char(c);
            }
            next_token = tokens.next();
            if next_token.is_some() {
                consume_char(' ');
            }
        }

        // Flushes current line, then finishes.
        result.lines.push(current_line);
        result
    }

    fn wrapping(
        string: &str,
        line_height: f32,
        offset_y: f32,
        max_width: f32,
        glyph_metrics: GlyphMetrics,
        charmap: Charmap
    ) -> Self {
        let mut result = Self::default();

        // Converts string to tokens
        let chars = strip_multiple_whitespace(string.chars());
        let mut tokens: Vec<Token> = Tokens::new(
            &mut result.glyphs,
            chars,
            charmap,
            glyph_metrics
        ).collect();

        // Converts tokens to lines
        let slice: &mut [Token] = &mut tokens;
        let mut current_line = Line::default();
        for i in 0..slice.len() {
            let token = &mut slice[i];
            let token_width = token.width;
            let token_max_x = token.x + token.width;
            let token_start = token.start;
            if token.newline || token_max_x > max_width {
                let off_x = -token.x;
                let off_y = line_height;
                shift_tokens(&mut slice[i..], off_x, off_y);
                result.lines.push(current_line);
                current_line.width = token_width;
                current_line.index = token_start;
            }
            else {
                current_line.width = token_max_x;
            }
        }
        result.lines.push(current_line);

        // Globalizes glyph coordinates
        for token in &mut tokens {
            token.y += offset_y;
            token.offset_glyphs(&mut result.glyphs);
            let max_x = token.x + token.width;
            let max_y = token.y + line_height;
            result.width = result.width.max(max_x);
            result.height = result.height.max(max_y);
        }
        result
    }
}

fn shift_tokens(tokens: &mut [Token], off_x: f32, off_y: f32) {
    for token in tokens {
        token.x += off_x;
        token.y += off_y;
    }
}

/// Filters character sequence such that multiple whitespace characters are filtered out.
/// IE:
///     Input:  'My     string'
///     Output: 'My string'
fn strip_multiple_whitespace(iter: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    let mut last_char_whitespace = false;
    iter.filter(move |c| {
        let c = *c;
        if c == ' ' {
            if last_char_whitespace {
                return false;
            }
            last_char_whitespace = true;
        }
        else if c == '\n' {
            last_char_whitespace = true;
        }
        else {
            last_char_whitespace = false;
        }
        true
    })
}

#[derive(Copy, Clone, Default, Debug)]
struct Line { index: usize, width: f32 }

#[derive(Copy, Clone, Default, Debug)]
struct Token {
    start: usize,
    end: usize,
    x: f32,
    y: f32,
    width: f32,
    newline: bool,
}

impl Token {
    fn offset_glyphs(&self, glyphs: &mut [Glyph]) {
        for i in self.start..self.end {
            let glyph = &mut glyphs[i];
            glyph.x += self.x;
            glyph.y += self.y;
        }
    }
}

struct Tokens<'a, I> {
    storage: &'a mut Vec<Glyph>,
    current_token: Token,
    pen_x: f32,
    pen_y: f32,
    chars: I,
    charmap: Charmap<'a>,
    metrics: GlyphMetrics<'a>,
    done: bool,
}

impl<'a, I> Tokens<'a, I> {
    pub fn new(
        storage: &'a mut Vec<Glyph>,
        chars: I,
        charmap: Charmap<'a>,
        metrics: GlyphMetrics<'a>,
    ) -> Self {
        Self {
            storage,
            current_token: Token::default(),
            pen_x: 0.0,
            pen_y: 0.0,
            chars,
            charmap,
            metrics,
            done: false,
        }
    }
}

impl<'a, I: Iterator<Item = char>> Iterator for Tokens<'a, I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }
        loop {
            let Some(c) = self.chars.next() else {
                self.done = true;
                return Some(self.current_token);
            };
            let gid = self.charmap.map(c).unwrap_or_default();
            let advance = self.metrics.advance_width(gid).unwrap_or_default();
            if c == '\n' {
                let token = self.current_token;
                self.pen_x += advance;
                self.current_token = Token {
                    start: self.current_token.end,
                    end: self.current_token.end,
                    x: self.pen_x,
                    y: self.pen_y,
                    width: 0.0,
                    newline: true,
                };
                return Some(token);
            }
            if c == ' ' {
                let token = self.current_token;
                let glyph = Glyph { id: gid.to_u32(), x: self.pen_x - self.current_token.x, y: self.pen_y - self.current_token.y };
                self.storage.push(glyph);
                self.pen_x += advance;
                self.current_token = Token {
                    start: self.current_token.end + 1,
                    end: self.current_token.end + 1,
                    x: self.pen_x,
                    y: self.pen_y,
                    width: 0.0,
                    newline: false,
                };
                return Some(token);
            }
            else {
                let glyph = Glyph { id: gid.to_u32(), x: self.pen_x - self.current_token.x, y: self.pen_y - self.current_token.y };
                self.storage.push(glyph);
                self.current_token.end += 1;
                self.current_token.width += advance;
                self.pen_x += advance;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::strip_multiple_whitespace;

    #[test]
    fn test_single_line() {
        let string = "This     is my    string";
        let expected = String::from("This is my string");
        let actual: String = strip_multiple_whitespace(string.chars()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multi_line() {
        let string = "This     is\n my    string";
        let expected = String::from("This is\nmy string");
        let actual: String = strip_multiple_whitespace(string.chars()).collect();
        assert_eq!(expected, actual);
    }
}