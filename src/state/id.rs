use std::marker::PhantomData;
use std::sync::mpsc::Sender;
use crate::{State, StateEvent};
use slotmap::new_key_type;

/// A typed identifier for a reference-counted [`State`] object within a [`Store`](crate::Store).
#[derive(Debug)]
pub struct Id<S: State> {
    untyped: UntypedId,
    phantom: PhantomData<S>,
}

impl<S: State> Id<S> {

    pub(crate) fn new(raw: RawId, events: Sender<StateEvent>) -> Self {
        Self {
            untyped: UntypedId { raw, kind: IdKind::Strong(events) },
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn untyped(&self) -> &UntypedId {
        &self.untyped
    }

    #[inline(always)]
    pub fn raw(&self) -> RawId {
        self.untyped.raw
    }

    pub fn clone_weak(&self) -> Self {
        Self {
            untyped: self.untyped.clone_weak(),
            phantom: PhantomData,
        }
    }
}

impl<S: State> Clone for Id<S> {
    fn clone(&self) -> Self {
        Self {
            untyped: self.untyped.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S: State> From<UntypedId> for Id<S> {
    fn from(untyped: UntypedId) -> Self {
        Self {
            untyped,
            phantom: PhantomData,
        }
    }
}


/// The untyped variant of [`Id`].
#[derive(Debug)]
pub struct UntypedId {
    raw: RawId,
    kind: IdKind,
}

impl UntypedId {

    #[inline(always)]
    pub fn raw(&self) -> RawId {
        self.raw
    }

    pub fn clone_weak(&self) -> Self {
        Self {
            raw: self.raw,
            kind: IdKind::Weak,
        }
    }
}

#[derive(Debug)]
pub enum IdKind {
    Weak,
    Strong(Sender<StateEvent>)
}

impl Clone for UntypedId {
    fn clone(&self) -> Self {
        match &self.kind {
            IdKind::Weak => Self {
                raw: self.raw,
                kind: IdKind::Weak,
            },
            IdKind::Strong(events) => {
                let _ = events.send(StateEvent::Clone(self.raw));
                Self {
                    raw: self.raw,
                    kind: IdKind::Strong(events.clone()),
                }
            },
        }
    }
}

impl Drop for UntypedId {
    fn drop(&mut self) {
        match &mut self.kind {
            IdKind::Weak => {},
            IdKind::Strong(events) => {
                let _ = events.send(StateEvent::Drop(self.raw));
            },
        }
    }
}


new_key_type! {
    pub struct RawId;
}