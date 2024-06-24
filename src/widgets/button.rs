use crate::{Class, EventCtx, GewyString, MouseButton, View, Widget, WidgetEvent, WidgetId};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub press: Option<Box<dyn Fn(&mut EventCtx)>>,
    pub release: Option<Box<dyn Fn(&mut EventCtx)>>,
}

impl Button {

    pub fn press(&mut self, press: impl Fn(&mut EventCtx) + 'static) -> &mut Self {
        self.press = Some(Box::new(press));
        self
    }

    pub fn release(&mut self, release: impl Fn(&mut EventCtx) + 'static) -> &mut Self {
        self.release = Some(Box::new(release));
        self
    }
}

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
                let Some(press) = &self.press else { return true };
                if mouse_button == MouseButton::Left && mouse_x >= 0.0 && mouse_y >= 0.0 && mouse_x <= width && mouse_y <= height {
                    press(&mut ctx);
                    return false;
                }
            },
            WidgetEvent::Released { mouse_button, mouse_x, mouse_y, width, height } => {
                let Some(release) = &self.release else { return true };
                if mouse_button == MouseButton::Left && mouse_x >= 0.0 && mouse_y >= 0.0 && mouse_x <= width && mouse_y <= height {
                    release(&mut ctx);
                    return false;
                }
            },
        }
        true
    }
}

/// Widget function for [`Button`].
pub fn button<'a>(class: impl Class<Button>, button_id: &mut WidgetId, v: &'a mut View) {
    let button = class.produce();
    let id = v.insert(button);
    *button_id = id;
}

/// Widget function for [`Button`].
pub fn button_begin<'a>(class: impl Class<Button>, button_id: &mut WidgetId, v: &'a mut View) {
    let button = class.produce();
    let id = v.insert(button);
    v.begin();
    *button_id = id;
}

/// Widget function to retrieve a [`Button`].
pub fn button_mut<'a>(widget_id: WidgetId, v: &'a mut View) -> &'a mut Button {
    v.widget_mut(widget_id)
}