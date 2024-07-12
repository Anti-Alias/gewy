use crate::{Class, InputMessage, Message, MessageType, MouseButton, Msg, Store, Text, ToUiString, View, Widget};
use crate::vello::Scene;
use crate::taffy::{Style, Layout};
use crate::peniko::{Color, Fill};
use crate::kurbo::{Affine, RoundedRect, RoundedRectRadii};

#[derive(Default)]
pub struct Button {
    pub style: Style,
    pub color: Color,
    pub radii: RoundedRectRadii,
    pub press: Option<Message>,
    pub release: Option<Message>,
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

    pub fn press(mut self, press: impl MessageType) -> Self {
        self.press = Some(press.into());
        self
    }

    pub fn release(mut self, release: impl MessageType) -> Self {
        self.release = Some(release.into());
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

    fn update<'a>(&'a self, _store: &mut Store, message: Msg<'a>) -> Option<Msg<'a>> {
        let Some(input) = message.downcast_ref::<InputMessage>() else {
            return Some(message);
        };
        match input {
            InputMessage::MousePressed { button: MouseButton::Left } => self.press.as_deref(),
            InputMessage::MouseReleased { button: MouseButton::Left } => self.release.as_deref(),
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

    pub fn press(mut self, press: impl MessageType) -> Self {
        self.button.press = Some(press.into());
        self
    }

    pub fn release(mut self, release: impl MessageType) -> Self {
        self.button.release = Some(release.into());
        self
    }

    pub fn insert(self, view: &mut View) {
        view.begin(self.button);
            view.insert(self.text);
        view.end();
    }
}