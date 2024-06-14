use taffy::Size;
use vello::kurbo::RoundedRectRadii;

use crate::{Widget, Scene, UIRenderer};
use crate::layout::{Style, Layout};
use crate::paint::{Color, Fill};
use crate::geom::{Affine, RoundedRect};

pub struct Div {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
}

impl Widget for Div {

    fn style(&self) -> Style { self.style.clone() }

    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout) {
        println!("{layout:#?}");
        let rect = RoundedRect::new(
            layout.location.x as f64,
            layout.location.y as f64,
            (layout.location.x + layout.size.width) as f64,
            (layout.location.y + layout.size.height) as f64,
            self.radii,
        );
        scene.fill(Fill::NonZero, Affine::IDENTITY, self.color, None, &rect);
    }

    #[allow(unused)]
    fn render(&self, r: &mut UIRenderer) {}
}

pub fn div(width: f32, height: f32, r: &mut UIRenderer) {
    r.insert(Div {
        style: Style {
            size: Size::from_lengths(width, height),
            ..Default::default()
        },
        color: Color::RED,
        radii: RoundedRectRadii::from_single_radius(3.0),
    });
}