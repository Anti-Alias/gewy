use crate::{Class, EventCtx, GewyString, Listener, MouseButton, View, Widget, WidgetEvent, WidgetId};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub listener: Option<Box<dyn Listener<ButtonEvent>>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonEvent { Pressed, Released }

impl Widget for Button {

    fn name(&self) -> GewyString { "button".into() }

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

    fn event(&self, event: WidgetEvent, mut ctx: EventCtx) -> bool {
        match event {
            WidgetEvent::Pressed { mouse_button, mouse_x, mouse_y, width, height } => {
                let Some(listener) = &self.listener else { return true };
                if mouse_button == MouseButton::Left && mouse_x >= 0.0 && mouse_y >= 0.0 && mouse_x <= width && mouse_y <= height {
                    listener.handle(ButtonEvent::Pressed, &mut ctx);
                    return false;
                }
            },
            WidgetEvent::Released { mouse_button, mouse_x, mouse_y, width, height } => {
                let Some(listener) = &self.listener else { return true };
                if mouse_button == MouseButton::Left && mouse_x >= 0.0 && mouse_y >= 0.0 && mouse_x <= width && mouse_y <= height {
                    listener.handle(ButtonEvent::Released, &mut ctx);
                    return false;
                }
            },
        }
        true
    }
}

/// Widget function for [`Button`].
pub fn button(class: impl Class<Button>, v: &mut View) -> WidgetId<Button> {
    let button = class.produce();
    v.insert(button)
}

/// Widget function for [`Button`].
pub fn button_begin<'a>(
    class: impl Class<Button>,
    listener: impl Listener<ButtonEvent>,
    v: &'a mut View
) {
    let mut button = Button {
        style: Style::DEFAULT,
        color: Color::rgba8(0, 0, 0, 0),
        radii: RoundedRectRadii::default(),
        listener: Some(Box::new(listener)),
    };
    class.apply(&mut button);
    v.insert(button);
    v.begin();
}
