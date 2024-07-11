use taffy::FlexDirection;
use crate::{Class, View, Widget};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default, Debug)]
pub struct Div {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
}

impl Widget for Div {

    fn name(&self) -> &str { "div" }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {
        if self.color.a == 0 { return };
        let rect = RoundedRect::new(
            layout.location.x as f64,
            layout.location.y as f64,
            (layout.location.x + layout.size.width) as f64,
            (layout.location.y + layout.size.height) as f64,
            self.radii,
        );
        scene.fill(Fill::NonZero, affine, self.color, None, &rect);
    }
}

impl Div {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn insert(self, view: &mut View) {
        view.insert(self);
    }

    #[inline(always)]
    pub fn begin(self, view: &mut View) {
        view.begin(self);
    }

    #[inline(always)]
    pub fn end(view: &mut View) {
        view.end();
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn radii(mut self, radii: RoundedRectRadii) -> Self {
        self.radii = radii;
        self
    }

    pub fn class(mut self, class: impl Class<Self>) -> Self {
        class.apply(&mut self);
        self
    }
}

/// A pseudo [`Widget`] that inserts a [`Div`].
/// Uses flexbox row layout by default.
pub struct Row(Div);
impl Row {

    pub fn new() -> Self {
        Self(Div::default())
    }

    #[inline(always)]
    pub fn begin(self, view: &mut View) {
        view.begin(self.0);
    }

    #[inline(always)]
    pub fn end(view: &mut View) {
        view.end();
    }

    pub fn style(mut self, style: Style) -> Self {
        self.0.style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }

    pub fn radii(mut self, radii: RoundedRectRadii) -> Self {
        self.0.radii = radii;
        self
    }

    pub fn class(mut self, class: impl Class<Div>) -> Self {
        class.apply(&mut self.0);
        self
    }
}

/// A pseudo [`Widget`] that inserts a [`Div`].
/// Uses flexbox column layout by default.
pub struct Col(Div);
impl Col {

    pub fn new() -> Self {
        let mut div = Div::default();
        div.style.flex_direction = FlexDirection::Column;
        Self(div)
    }

    #[inline(always)]
    pub fn begin(self, view: &mut View) {
        view.begin(self.0);
    }

    #[inline(always)]
    pub fn end(view: &mut View) {
        view.end();
    }

    pub fn style(mut self, style: Style) -> Self {
        self.0.style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }

    pub fn radii(mut self, radii: RoundedRectRadii) -> Self {
        self.0.radii = radii;
        self
    }

    pub fn class(mut self, class: impl Class<Div>) -> Self {
        class.apply(&mut self.0);
        self
    }
}