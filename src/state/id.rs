use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::mpsc::Sender;
use crate::{RawId, State, StateEvent};

/// A typed identifier for a reference-counted [`State`] object within a [`Store`](crate::Store).
#[derive(Debug)]
pub struct Handle<S: State> {
    id: Id<S>,
    events: Sender<StateEvent>,
}


impl<S: State> Deref for Handle<S> {
    type Target = Id<S>;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<S: State> Handle<S> {

    pub(crate) fn new(id: Id<S>, events: Sender<StateEvent>) -> Self {
        Self { id, events }
    }

    #[inline(always)]
    pub fn id(&self) -> Id<S> {
        self.id
    }
}

impl<S: State> Clone for Handle<S> {
    fn clone(&self) -> Self {
        let _ = self.events.send(StateEvent::Clone(self.id.raw));
        Self {
            id: self.id,
            events: self.events.clone(),
        }
    }
}

impl<S: State> Drop for Handle<S> {
    fn drop(&mut self) {
        let _ = self.events.send(StateEvent::Drop(self.id.raw));
    }
}

/// Typed identifier for a state object.
/// Similar to [`Handle`], but does not keep underlying resource alive.
/// Useful for view functions, as it implements [`Copy`], unlike [`Handle`], so it can be trivially sent to callback functions.
#[derive(Debug)]
pub struct Id<S: State> {
    raw: RawId,
    phantom: PhantomData::<S>,
}

impl<S: State> From<RawId> for Id<S> {
    fn from(raw_id: RawId) -> Self {
        Self {
            raw: raw_id,
            phantom: PhantomData,
        }
    }
}

impl<S: State> Id<S> {
    pub fn raw(&self) -> RawId { self.raw }
}

impl<S: State> Clone for Id<S> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
            phantom: PhantomData,
        }
    }
}

impl<S: State> Copy for Id<S> {}