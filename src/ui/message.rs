use std::any::Any;
use std::ops::{Deref, DerefMut};
use downcast_rs::{impl_downcast, Downcast};


/// A message emitted by either a [`Widget`](crate::Widget) or a [`State`](crate::State).
pub trait MessageType: Any + Downcast {}
impl_downcast!(MessageType);


/// A message sent to a [`Widget`](crate::Widget) for it to handle.
pub struct Message(Box<dyn MessageType>);

impl<M: MessageType> From<M> for Message {
    fn from(message: M) -> Self {
        Self(Box::new(message))
    }
}

impl Deref for Message {
    type Target = dyn MessageType;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl Message {

    pub fn as_ref(&self) -> Msg<'_> {
        self.0.as_ref()
    }
    pub fn downcast_ref<M: MessageType>(&self) -> Option<&M> {
        self.0.downcast_ref()
    }
}

/// A borrowed [Message].
pub type Msg<'a> = &'a dyn MessageType;
