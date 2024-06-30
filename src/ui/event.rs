use dyn_clone::DynClone;
use downcast_rs::{Downcast, impl_downcast};
use crate::{RawWidgetId, Store};

pub struct EventCtx<'a> {
    pub widget_id: RawWidgetId,
    pub store: &'a mut Store,
    pub(crate) messages: &'a mut Vec<WidgetMessage>,
}

impl<'a> EventCtx<'a> {
    pub fn emit(&mut self, message: DynMessage) {
        self.messages.push(WidgetMessage {
            widget_id: self.widget_id,
            message: message,
        });
    }
}

/// A message fired from a widget.
pub(crate) struct WidgetMessage {
    pub widget_id: RawWidgetId,
    pub message: DynMessage,
}

/// A message emitted by either a [`Widget`](crate::Widget) or a [`State`](crate::State).
pub trait Message: Downcast + DynClone + 'static {}
impl<T: Downcast + DynClone + 'static> Message for T {}
impl_downcast!(Message);
dyn_clone::clone_trait_object!(Message);


/// Dynamic form of a [`Message`].
#[derive(Clone)]
pub struct DynMessage(Box<dyn Message>);

impl DynMessage {
    pub fn new(message: impl Message) -> Self {
        Self(Box::new(message))
    }
    pub fn downcast_ref<M: Message>(&self) -> Option<&M> {
        self.0.downcast_ref()
    }
}
