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
pub fn div(class: impl Class<Div>, v: &mut View) -> WidgetId {
    let div = class.produce();
    v.insert(div)
}

/// Widget function for [`Div`].
pub fn div_begin(class: impl Class<Div>, v: &mut View) -> WidgetId {
    let div = class.produce();
    let id = v.insert(div);
    v.begin();
    id
}