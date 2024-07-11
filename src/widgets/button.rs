use crate::{Class, DynMessage, InputMessage, Message, MouseButton, Store, Text, ToUiString, View, Widget};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub press: Option<DynMessage>,
    pub release: Option<DynMessage>,
}

impl Button {

    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn insert(self, view: &mut View) {
        view.insert(self);
    }

    #[inline(always)]
    pub fn begin(self, view: &mut View) {
        view.begin(self);
    }

    #[inline(always)]
    pub fn end(view: &mut View) {
        view.end();
    }


    pub fn class(mut self, class: impl Class<Self>) -> Self {
        class.apply(&mut self);
        self
    }

    pub fn press(mut self, press: impl Message) -> Self {
        self.press = Some(DynMessage::new(press));
        self
    }

    pub fn release(mut self, release: impl Message) -> Self {
        self.release = Some(DynMessage::new(release));
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
        match input {
            InputMessage::MousePressed { button: MouseButton::Left } => self.press.clone(),
            InputMessage::MouseReleased { button: MouseButton::Left } => self.release.clone(),
            _ => None
        }
    }
}


#[derive(Default)]
pub struct TextButton {
    pub button: Button,
    pub text: Text,
}

impl TextButton {

    pub fn new(string: impl ToUiString) -> Self {
        let mut text = Text::default();
        text.string = string.to_ui_string();
        Self { button: Button::default(), text }
    }

    pub fn class(mut self, class: impl Class<Self>) -> Self {
        class.apply(&mut self);
        self
    }

    pub fn press(mut self, press: impl Message) -> Self {
        self.button.press = Some(DynMessage::new(press));
        self
    }

    pub fn release(mut self, release: impl Message) -> Self {
        self.button.release = Some(DynMessage::new(release));
        self
    }

    pub fn insert(self, view: &mut View) {
        view.begin(self.button);
            view.insert(self.text);
        view.end();
    }
}