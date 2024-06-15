use crate::{Class, Scene, UIRenderer, Widget};
use crate::layout::{Style, Layout};
use crate::paint::{Color, Fill};
use crate::geom::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default, Debug)]
pub struct Div {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
}

impl Widget for Div {

    fn style(&self) -> Style {
        self.style.clone()
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
    fn render(&self, r: &mut UIRenderer) {}
}

/// Widget function for [`Div`].
pub fn div(class: impl Class<Div>, r: &mut UIRenderer) {
    let mut div = Div::default();
    class.apply(&mut div);
    r.insert(div);
}