use crate::{Class, DynMapper, DynMessage, InputMessage, Mapper, MouseButton, Store, ViewCmds, Widget};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub mapper: DynMapper,
}

impl Button {
    #[inline(always)]
    pub fn insert(self, view: &mut ViewCmds) {
        view.insert(self);
    }

    #[inline(always)]
    pub fn begin(self, view: &mut ViewCmds) {
        view.begin(self);
    }

    pub fn map(mut self, mapper: impl Mapper) -> Self {
        self.mapper = DynMapper::from(mapper);
        self
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

    fn update(&self, _store: &mut Store, message: DynMessage) -> Option<DynMessage> {
        let Some(input) = message.downcast_ref::<InputMessage>() else {
            return Some(message);
        };
        let event = match input {
            InputMessage::MousePressed { button: MouseButton::Left } => ButtonEvent::Pressed,
            InputMessage::MouseReleased { button: MouseButton::Left } => ButtonEvent::Released,
            _ => return Some(message)
        };
        self.mapper.map(&event)
    }
}

/// Widget function for [`Button`].
pub fn button(class: impl Class<Button>) -> Button {
    class.produce()
}
