use std::any::Any;
use std::marker::PhantomData;
use std::sync::mpsc::{Receiver, Sender};
use slotmap::{new_key_type, SlotMap};


/// A handle to a piece of global state.
pub struct Store {
    states: SlotMap<RawId, StateData>,
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

    pub fn get<S: State>(&self, state: Id<S>) -> &S {
        let data = self.states.get(state.raw).unwrap();
        data.state.downcast_ref().unwrap()
    }

    pub fn get_mut<S: State>(&mut self, id: Id<S>) -> &mut S {
        self.updated_states.push(id.raw);
        let state_data = self.states.get_mut(id.raw).unwrap();
        let state_data: &mut StateData = unsafe { std::mem::transmute(state_data) };
        state_data.bindings.retain_mut(|binding| {
            if self.states.contains_key(*binding) {
                self.updated_states.push(*binding);
                true
            }
            else {
                false
            }
        });
        for bound_state_id in &state_data.bindings {
            self.updated_states.push(*bound_state_id);
        };
        state_data.state.downcast_mut().unwrap()
    }

    /// Creates a [`State`] with an initial value.
    pub fn create<S: State>(&mut self, state: S) -> Handle<S> {
        let mut bindings = vec![];
        state.bind(&mut StateBindings { bindings: &mut bindings });
        let state_data = StateData {
            ref_count: 1,
            bindings: vec![],
            state: Box::new(state),
        };
        let state_id = self.states.insert(state_data);
        for other_id in bindings {
            let other_state_data = self.states.get_mut(other_id).unwrap();
            other_state_data.add_binding(state_id);
        }
        Handle { id: state_id, events: self.sender.clone(), phantom: PhantomData }
    }

    fn cascade_updates<S: State>(&mut self, id: Id<S>) {
        let state_data = self.states.get_mut(id.raw).unwrap();
        let state = state_data.state.downcast_mut::<S>().unwrap();
        let mut bindings = vec![];
        state.bind(&mut StateBindings { bindings: &mut bindings });
        for other_id in bindings {
            let other_state_data = self.states.get_mut(other_id).unwrap();
            other_state_data.add_binding(id.raw);
        }
    }

    /// Creates a [`State`] whose initial value is derived from the [`Store`].
    pub fn init<S: State + FromStore>(&mut self) -> Handle<S> {
        let value = S::from_store(self);
        self.create(value)
    }

    /// Handles buffered events.
    /// Returns ids of all states that updated since the last invocation.
    pub(crate) fn update(&mut self) -> Vec<RawId> {
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
        self.take_state_changes()

    }

    fn take_state_changes(&mut self) -> Vec<RawId> {
        let immediate_updates = std::mem::take(&mut self.updated_states);
        for state_id in immediate_updates {

        }
        todo!()
    }
}

pub trait State: Any {
    #[allow(unused)]
    fn bind(&self, bindings: &mut StateBindings) {}
}

pub struct StateBindings<'a> {
    bindings: &'a mut Vec<RawId>,
}

impl<'a> StateBindings<'a> {
    pub fn add<S: State>(&mut self, state_id: &Handle<S>) {
        self.bindings.push(state_id.id().raw);
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
/// Similar to [`Id`], but keeps the underlying resource alive until dropped.
#[derive(Debug)]
pub struct Handle<S: State> {
    id: RawId,
    events: Sender<StateEvent>,
    phantom: PhantomData::<S>,
}

impl<S: State> Handle<S> {
    pub fn id(&self) -> Id<S> {
        Id {
            raw: self.id.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S: State> Clone for Handle<S> {
    fn clone(&self) -> Self {
        let _ = self.events.send(StateEvent::Clone(self.id));
        Self {
            id: self.id,
            events: self.events.clone(),
            phantom: PhantomData,
        }
    }
}

impl<S: State> Drop for Handle<S> {
    fn drop(&mut self) {
        let _ = self.events.send(StateEvent::Drop(self.id));
    }
}


/// Typed identifier for some state.
/// Similar to [`Id`], but keeps the underlying resource alive until dropped.
#[derive(Debug)]
pub struct Id<S: State> {
    raw: RawId,
    phantom: PhantomData::<S>,
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



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StateEvent {
    Clone(RawId),
    Drop(RawId),
}

/// A container for state within a [`Store`].
struct StateData {
    ref_count: u32,
    bindings: Vec<RawId>,
    state: Box<dyn Any>,
}

impl StateData {
    pub fn add_binding(&mut self, id: RawId) {
        for existing_id in &self.bindings {
            if *existing_id == id { return }
        }
        self.bindings.push(id);
        println!("Added binding???");
    }
}


new_key_type! {
    /// The untyped variant of [`Id`].
    pub struct RawId;
}