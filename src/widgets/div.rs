use taffy::FlexDirection;

use crate::{Class, GewyString, View, Widget, WidgetId};
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

    fn name(&self) -> GewyString { "div".into() }

    fn disable_view(&self) -> bool { true }

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
pub fn div(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    let div = class.produce();
    v.insert(div)
}

/// Widget function for [`Div`].
pub fn div_begin(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    let div = class.produce();
    let id = v.insert(div);
    v.begin();
    id
}

/// Displays contents left to right. Alias for [`div`].
#[inline(always)]
pub fn row(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    div(class, v)
}

/// Displays contents left to right. Alias for [`div_begin`].
#[inline(always)]
pub fn row_begin(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    div_begin(class, v)
}

/// Alias for [`div`]. Displays contents top to bottom.
pub fn col(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    let mut d = Div::default();
    class.apply(&mut d);
    d.style.flex_direction = FlexDirection::Column;
    v.insert(d)
}

/// Alias for [`div_begin`]. Displays contents top to bottom.
#[inline(always)]
pub fn col_begin(class: impl Class<Div>, v: &mut View) -> WidgetId<Div> {
    let mut d = Div::default();
    class.apply(&mut d);
    d.style.flex_direction = FlexDirection::Column;
    let id = v.insert(d);
    v.begin();
    id
}