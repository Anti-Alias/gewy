use crate::{Class, WidgetId, Scene, Renderer, Widget};
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

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {
        let rect = RoundedRect::new(
            layout.location.x as f64,
            layout.location.y as f64,
            (layout.location.x + layout.size.width) as f64,
            (layout.location.y + layout.size.height) as f64,
            self.radii,
        );
        scene.fill(Fill::NonZero, affine, self.color, None, &rect);
    }

    #[allow(unused)]
    fn render(&self, r: &mut Renderer) {}
}

/// Widget function for [`Div`].
pub fn div(class: impl Class<Div>, r: &mut Renderer) -> WidgetId {
    let div = class.produce();
    r.insert(div)
}

pub fn div_begin(class: impl Class<Div>, r: &mut Renderer) -> WidgetId {
    let div = class.produce();
    let id = r.insert(div);
    r.begin();
    id
}