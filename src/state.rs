use std::any::Any;
use std::marker::PhantomData;
use std::sync::mpsc::{Receiver, Sender};
use slotmap::{new_key_type, SlotMap};

use crate::EventCtx;


/// A handle to a piece of global state.
pub struct Store {
    datum: SlotMap<RawId, StateData>,
    sender: Sender<StateEvent>,
    receiver: Receiver<StateEvent>,
    updated_states: Vec<RawId>,
}

impl Store {

    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            datum: SlotMap::default(),
            sender,
            receiver,
            updated_states: vec![],
        }
    }

    pub fn get<S: Any>(&self, state: &Id<S>) -> &S {
        let data = self.datum.get(state.handle).unwrap();
        data.value.downcast_ref().unwrap()
    }

    pub fn get_mut<S: Any>(&mut self, state: &Id<S>) -> &mut S {
        let data = self.datum.get_mut(state.handle).unwrap();
        self.updated_states.push(state.handle);
        data.value.downcast_mut().unwrap()
    }

    /// Creates a [`State`] with an initial value.
    pub fn create<S: Any>(&mut self, value: S) -> Id<S> {
        let data = StateData {
            ref_count: 1,
            value: Box::new(value),
        };
        let id = self.datum.insert(data);
        Id {
            handle: id,
            events: self.sender.clone(),
            phantom: PhantomData,
        }
    }

    /// Creates a [`State`] whose initial value is derived from the [`Store`].
    pub fn init<S: Any + FromStore>(&mut self) -> Id<S> {
        let value = S::from_store(self);
        self.create(value)
    }

    /// Handles buffered events.
    /// Returns ids of all states that updated since the last invocation.
    pub(crate) fn update(&mut self) -> Vec<RawId> {
        for event in self.receiver.try_iter() {
            match event {
                StateEvent::Clone(id) => {
                    let data = self.datum.get_mut(id).unwrap();
                    data.ref_count += 1;
                },
                StateEvent::Drop(id) => {
                    let data = self.datum.get_mut(id).unwrap();
                    data.ref_count -= 1;
                    if data.ref_count == 0 {
                        self.datum.remove(id);
                    }
                },
            }
        }
        std::mem::take(&mut self.updated_states)
    }
}

/// Any state type that can be derived from a [`Store`].
pub trait FromStore {
    fn from_store(store: &mut Store) -> Self;
}

impl<D: Default> FromStore for D {
    fn from_store(_store: &mut Store) -> Self {
        Self::default()
    }
}

/// Typed identifier for some state.
#[derive(Debug)]
pub struct Id<S: Any> {
    pub handle: RawId,
    pub events: Sender<StateEvent>,
    phantom: PhantomData::<S>,
}

impl<S: Any> Id<S> {

    /// Helper method that sets a value in the store.
    #[inline(always)]
    pub fn set(self, value: S, store: &mut Store) {
        *store.get_mut(&self) = value;
    }

    /// Helper method for getting a mutable value.
    #[inline(always)]
    pub fn get_mut<'a>(&self, ctx: &'a mut EventCtx) -> &'a mut S {
        ctx.store.get_mut(self)
    }
}

impl<S: Any> AsRef<Id<S>> for Id<S> {
    fn as_ref(&self) -> &Self { self }
}

impl<S: Any> Clone for Id<S> {
    fn clone(&self) -> Self {
        let _ = self.events.send(StateEvent::Clone(self.handle));
        Self {
            handle: self.handle,
            events: self.events.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S: Any> Drop for Id<S> {
    fn drop(&mut self) {
        let _ = self.events.send(StateEvent::Drop(self.handle));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StateEvent {
    Clone(RawId),
    Drop(RawId),
}

/// A container for state within a [`Store`].
struct StateData {
    ref_count: u32,
    value: Box<dyn Any>,
}


new_key_type! {
    /// The untyped variant of [`Id`].
    pub struct RawId;
}