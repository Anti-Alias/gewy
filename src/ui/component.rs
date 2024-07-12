use crate::{Id, MessageType, Msg, State, Store, UntypedId, View, Widget};


pub trait Component: Sized + 'static {

    type State: State;
    type Message: MessageType;

    fn state(&self) -> &Id<Self::State>;

    fn name(&self) -> &str { "component" }

    #[allow(unused)]
    fn update(&self, params: UParams<Self>);

    #[allow(unused)]
    fn view(&self, params: VParams<Self>);
}

pub(crate) struct Comp<C: Component>(pub C);

impl<C: Component> Widget for Comp<C> {

    fn name(&self) -> &str { self.0.name() }

    fn update<'a>(&'a self, store: &mut Store, msg: Msg<'a>) -> Option<Msg<'a>> {
        let Some(msg) = msg.downcast_ref::<C::Message>() else { return Some(msg) };
        let state_id = self.0.state();
        let mut state = store.remove(state_id);
        let params = UParams::<C> { state: &mut state, msg, store };
        self.0.update(params);
        store.restore(state_id, state);
        None
    }

    fn state_id(&self) -> Option<&UntypedId> {
        Some(self.0.state().untyped())
    }

    fn view(&self, store: &Store, view: &mut View) {
        let state = store.get(&self.0.state());
        let params = VParams::<C> { state, store, view };
        self.0.view(params);
    }
}


/// Update parameters
pub struct UParams<'a, C: Component> {
    pub state: &'a mut C::State,
    pub msg: &'a C::Message,
    pub store: &'a mut Store,
}

impl<'a, C: Component> UParams<'a, C> {
    pub fn unpack(&mut self) -> (&mut C::State, &C::Message, &Store) {
        (self.state, self.msg, self.store)
    }
}

/// View parameters
pub struct VParams<'a, C: Component> {
    pub state: &'a C::State,
    pub store: &'a Store,
    pub view: &'a mut View,
}

impl<'a, C: Component> VParams<'a, C> {
    pub fn unpack(&mut self) -> (&C::State, &mut View, &Store) {
        (self.state, self.view, self.store)
    }
}