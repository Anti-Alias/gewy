use std::sync::mpsc::{Receiver, Sender};
use downcast_rs::{impl_downcast, Downcast};
use slotmap::SlotMap;
use crate::{Id, RawId};


/// Storage for reference-counted [`State`] objects.
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
        let obj = self.states.get(id.raw()).unwrap();
        match &obj.slot {
            Slot::Occupied(state) => state.downcast_ref().unwrap(),
            Slot::Empty => panic!("Slot not occupied"),
        }
    }

    pub fn get_mut<S: State>(&mut self, id: &Id<S>) -> &mut S {
        let obj = self.states.get_mut(id.raw()).unwrap();
        match &mut obj.slot {
            Slot::Occupied(state) => state.downcast_mut().unwrap(),
            Slot::Empty => panic!("Slot not occupied"),
        }
    }

    pub(crate) fn remove<S: State>(&mut self, id: &Id<S>) -> S {
        self.updated_states.push(id.raw());
        let obj = self.states.get_mut(id.raw()).unwrap();
        let slot = std::mem::replace(&mut obj.slot, Slot::Empty);
        let value = match slot {
            Slot::Occupied(value) => value,
            Slot::Empty => panic!("Empty slot"),
        };
        let value: Box<S> = match value.downcast() {
            Ok(value) => value,
            Err(_) => panic!("Failed to downcast"),
        };
        *value
    }

    pub(crate) fn restore<S: State>(&mut self, id: &Id<S>, state: S) {
        let obj = self.states.get_mut(id.raw()).unwrap();
        obj.slot = Slot::Occupied(Box::new(state));
    }

    /// Creates a [`State`] with an initial value.
    pub fn create<S: State>(&mut self, state: S) -> Id<S> {
        let raw_id = self.states.insert(StateObject::new(state));
        Id::new(raw_id, self.sender.clone())
    }

    /// Creates a [`State`] whose initial value is derived from the [`Store`].
    pub fn init<S: State + FromStore>(&mut self) -> Id<S> {
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

/// A marker trait for the state of a component.
pub trait State: Downcast {}
impl_downcast!(State);
impl<T: Downcast> State for T {}


/// Any [`State`] type that can be derived from a [`Store`].
/// Useful for [`State`]s that have a default value with fields
/// that are also [`State`].
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

/// A reference-counted state object.
struct StateObject {
    ref_count: u32,
    slot: Slot,
}

impl StateObject {
    fn new(state: impl State) -> Self {
        Self {
            ref_count: 1,
            slot: Slot::Occupied(Box::new(state)),
        }
    }
}

pub enum Slot {
    Occupied(Box<dyn State>),
    Empty,
}