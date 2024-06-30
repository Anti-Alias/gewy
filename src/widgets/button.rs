use crate::{Class, DynMessage, InputEvent, EventCtx, Message, MouseButton, View, Widget};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub on_press: Option<DynMessage>,
    pub on_release: Option<DynMessage>
}

impl Button {
    pub fn on_press(&mut self, message: impl Message) {
        self.on_press = Some(DynMessage::new(message));
    }
    pub fn on_release(&mut self, message: impl Message) {
        self.on_release = Some(DynMessage::new(message));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ButtonEvent { Pressed, Released }

impl Widget for Button {

    fn name(&self) -> &str { "button" }

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

    fn event(&self, event: InputEvent, mut ctx: EventCtx) -> bool {
        match event {
            InputEvent::MousePressed { button: MouseButton::Left } => {
                if let Some(press_event) = &self.on_press {
                    ctx.emit(press_event.clone());
                }
            },
            InputEvent::MouseReleased { button: MouseButton::Left } => {
                if let Some(release_event) = &self.on_release {
                    ctx.emit(release_event.clone());
                }
            },
            _ => {},
        }
        true
    }
}

/// Widget function for [`Button`].
pub fn button(class: impl Class<Button>, v: &mut View) {
    let mut button = Button {
        style: Style::DEFAULT,
        color: Color::rgba8(0, 0, 0, 0),
        radii: RoundedRectRadii::default(),
        on_press: None,
        on_release: None,
    };
    class.apply(&mut button);
    v.insert(button);
}

/// Widget function for [`Button`].
pub fn button_begin(class: impl Class<Button>, v: &mut View) {
    button(class, v);
    v.begin();
}
