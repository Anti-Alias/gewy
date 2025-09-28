use std::{any::Any, collections::HashMap};
use smallvec::{smallvec, SmallVec};

/// Storage of state objects.
pub(crate) struct StateManager {
    states: HashMap<StateId, StateCell>,
}


impl StateManager {

    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Inserts a new state.
    pub fn insert(&mut self, id: StateId, state: impl Any + 'static) -> Option<Box<dyn Any>> {
        self.states
            .insert(id, StateCell::new(state))
            .map(|cell| cell.value)
    }

    /// Removes an existing.
    pub fn remove(&mut self, id: &StateId) -> Option<Box<dyn Any>> {
        self.states.remove(&id).map(|cell| cell.value)
    }

    /// Gets a state. 
    pub fn get(&self, id: &StateId) -> Option<&dyn Any> {
        self.states
            .get(id)
            .map(|cell| cell.value.as_ref())
    }

    /// Mutably gets a state cell by id, triggering change detection.
    pub fn get_mut(&mut self, id: &StateId) -> Option<&mut dyn Any> {
        let cell = self.states.get_mut(id)?;
        cell.changed = true;
        Some(&mut cell.value)
        
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn detect_changes<F>(&mut self, mut handle: F) 
    where
        F: FnMut(&StateId, &dyn Any),
    {
        for (state_id, state_cell) in &mut self.states {
            if !state_cell.changed { continue };
            handle(state_id, &state_cell.value);
            state_cell.changed = false;
        }
    }
}

/// Stores a single state object.
/// Tracks changes. 
struct StateCell {
    value: Box<dyn Any>,
    changed: bool,
}

impl StateCell {
    fn new(value: impl Any) -> Self {
        Self {
            value: Box::new(value),
            changed: true,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StateId(SmallVec::<[u16; 4]>);
impl StateId {
    fn child(&self, child_id: u16) -> Self {
        let mut id = self.0.clone();
        id.push(child_id);
        Self(id)
    }
}

impl From<u16> for StateId {
    fn from(value: u16) -> Self {
        Self(smallvec![value])
    }
}

impl From<&[u16]> for StateId {
    fn from(value: &[u16]) -> Self {
        Self(SmallVec::from_slice(value))
    }
}

