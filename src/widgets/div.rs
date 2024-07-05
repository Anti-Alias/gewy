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

/// Widget function for [`Div`].
pub fn div<'a, 'b>(class: impl Class<Div>, v: &'a mut View<'b>) -> &'a mut View<'b> {
    let div = class.produce();
    v.insert(div);
    v
}

/// Displays contents left to right. Alias for [`div`].
#[inline(always)]
pub fn row<'a, 'b>(class: impl Class<Div>, v: &'a mut View<'b>) -> &'a mut View<'b> {
    div(class, v)
}


/// Alias for [`div`]. Displays contents top to bottom.
pub fn col<'a, 'b>(class: impl Class<Div>, v: &'a mut View<'b>) -> &'a mut View<'b> {
    let mut d = Div::default();
    class.apply(&mut d);
    d.style.flex_direction = FlexDirection::Column;
    v.insert(d);
    v
}