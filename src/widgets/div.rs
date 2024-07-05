use taffy::FlexDirection;

use crate::{Class, View, Widget, WidgetId};
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

impl Div {

    #[inline(always)]
    pub fn ins(self, v: &mut View) -> WidgetId<Self> {
        v.insert(self)
    }

    #[inline(always)]
    pub fn beg(self, v: &mut View) -> WidgetId<Self> {
        let id = v.insert(self);
        v.begin();
        id
    }
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

/// Widget function for [`Div`].
pub fn div(class: impl Class<Div>) -> Div {
    class.produce()
}

/// Displays contents left to right. Alias for [`div`].
#[inline(always)]
pub fn row(class: impl Class<Div>) -> Div {
    class.produce()
}


/// Alias for [`div`]. Displays contents top to bottom.
pub fn col(class: impl Class<Div>) -> Div {
    let mut d = Div::default();
    d.style.flex_direction = FlexDirection::Column;
    class.apply(&mut d);
    d
}