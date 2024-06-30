use std::ops::DerefMut;
use std::sync::mpsc::{Receiver, Sender};
use downcast_rs::{impl_downcast, Downcast};
use slotmap::{new_key_type, SlotMap};

use crate::{DynMessage, Handle, Id};


/// Storage for state objects.
pub struct Store {
    states: SlotMap<RawId, StateObject>,
    sender: Sender<StateEvent>,
    receiver: Receiver<StateEvent>,
    updated_states: Vec<RawId>,
}

impl Store {

    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            states: SlotMap::default(),
            sender,
            receiver,
            updated_states: vec![],
        }
    }

    pub fn get<S: State>(&self, id: &Id<S>) -> &S {
        let data = self.states.get(id.raw()).unwrap();
        data.state.downcast_ref().unwrap()
    }

    pub fn get_mut<S: State>(&mut self, id: &Id<S>) -> &mut S {
        self.updated_states.push(id.raw());
        let state_data = self.states.get_mut(id.raw()).unwrap();
        state_data.state.downcast_mut().unwrap()
    }

    pub(crate) fn get_mut_dyn(&mut self, id: RawId) -> &mut dyn State {
        let state_obj = self.states.get_mut(id).unwrap();
        state_obj.state.deref_mut()
    }

    /// Creates a [`State`] with an initial value.
    pub fn create<S: State>(&mut self, state: S) -> Handle<S> {
        let raw_id = self.states.insert(StateObject {
            ref_count: 1,
            state: Box::new(state),
        });
        let id = Id::from(raw_id);
        Handle::new(id, self.sender.clone())
    }

    /// Creates a [`State`] whose initial value is derived from the [`Store`].
    pub fn init<S: State + FromStore>(&mut self) -> Handle<S> {
        let state = S::from_store(self);
        self.create(state)
    }

    /// Handles buffered events.
    /// Returns ids of all states that updated since the last invocation.
    pub(crate) fn handle_events(&mut self) -> Vec<RawId> {
        for event in self.receiver.try_iter() {
            match event {
                StateEvent::Clone(id) => {
                    let data = self.states.get_mut(id).unwrap();
                    data.ref_count += 1;
                },
                StateEvent::Drop(id) => {
                    let data = self.states.get_mut(id).unwrap();
                    data.ref_count -= 1;
                    if data.ref_count == 0 {
                        self.states.remove(id);
                    }
                },
            }
        }
        std::mem::take(&mut self.updated_states)
    }
}

/// A view into a [`Store`].
pub struct StoreView<'a> {
    store: &'a mut Store,
    updating_id: RawId,
}

impl<'a> StoreView<'a> {
    pub fn get<S: State>(&self, id: &Id<S>) -> &S {
        self.check_id(id.raw());
        self.store.get(id)
    }

    pub fn get_mut<S: State>(&mut self, id: &Id<S>) -> &mut S {
        self.check_id(id.raw());
        self.store.get_mut(id)
    }

    fn check_id(&self, id: RawId) {
        if id == self.updating_id {
            panic!("State being updated");
        }
    }
}

pub trait State: Downcast {}
impl_downcast!(State);
impl<T: Downcast> State for T {}


/// Any state type that can be derived from a [`Store`].
pub trait FromStore {
    fn from_store(store: &mut Store) -> Self;
}

impl<D: Default> FromStore for D {
    fn from_store(_store: &mut Store) -> Self {
        Self::default()
    }
}



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StateEvent {
    Clone(RawId),
    Drop(RawId),
}

/// A container for state within a [`Store`].
struct StateObject {
    ref_count: u32,
    state: Box<dyn State>,
}


new_key_type! {
    /// The untyped variant of [`Id`].
    pub struct RawId;
}