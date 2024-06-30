use crate::{Class, DynMessage, GewyString, Handle, Id, RawId, State, Store, View, Widget};
use crate::taffy::Style;

/// An inline [`Widget`] not bound to any state.
/// Descendants are generated immediately after insertion.
/// Useful as a "root widget" in an application.
pub struct Wid<V> {
    pub style: Style,
    pub view_fn: V,
}

impl<V> Widget for Wid<V>
where
    V: ViewFn,
{
    fn name(&self) -> GewyString {
        "inline".into()
    }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    #[allow(unused)]
    fn view(&self, _store: &Store, v: &mut View) {
        let view_fn = &self.view_fn;
        view_fn.view(v);
    }
}

impl<V> Wid<V>
where
    V: ViewFn,
{
    pub fn new(view_fn: V, class: impl Class<Style>) -> Self {
        Self {
            style: class.produce(),
            view_fn,
        }
    }
}

/// A "component" is an inline [`Widget`] bound to some state.
/// Its descendants are populated using a view function which takes its state as an argument.
/// Descendants are generated immediately after insertion.
/// Descendants are regenerated whenever the state changes.
/// Useful as a "root widget" in an application.
struct Comp<S, R, V>
where
    S: State,
    R: ReducerFn<S>,
    V: StateViewFn<S>,
{
    pub state: Handle<S>,
    pub reducer: R,
    pub view: V,
}

impl<S, R, V> Widget for Comp<S, R, V>
where
    S: State,
    R: ReducerFn<S>,
    V: StateViewFn<S>,
{

    fn name(&self) -> GewyString {
        "comp".into()
    }

    fn state_id(&self) -> Option<RawId> {
        Some(self.state.id().raw())
    }

    fn reduce_state(&self, state_id: RawId, store: &mut Store, message: DynMessage) {
        let reducer = &self.reducer;
        reducer.reduce(Id::from(state_id), store, message);
    }

    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        let view_fn = &self.view;
        view_fn.view(self.state.id(), store, v);
    }
}

impl<S, R, V> Comp<S, R, V>
where
    S: State,
    R: ReducerFn<S>,
    V: StateViewFn<S>,
{
    fn new(state: Handle<S>, reducer: R, view: V) -> Self {
        Self {
            state,
            reducer,
            view,
        }
    }
}

pub fn make_comp<S, R, V>(
    state: Handle<S>,
    reducer: R,
    view_fn: V
) -> impl Widget
where
    S: State,
    R: ReducerFn<S>,
    V: StateViewFn<S>,
{
    Comp::new(state, reducer, view_fn)
}

/// A callback that builds the descendants of a [`Widget`].
pub trait ViewFn: 'static {
    fn view(&self, view: &mut View);
}

impl<F> ViewFn for F
where
    F: Fn(&mut View) + 'static,
{
    fn view(&self, view: &mut View) {
        self(view)
    }
}

/// A callback that builds the descendants of a [`Widget`] with respect to some state.
pub trait StateViewFn<S: State>: 'static {
    fn view(&self, state_id: Id<S>, store: &Store, view: &mut View);
}

impl<S: State, F> StateViewFn<S> for F
where
    F: Fn(Id<S>, &Store, &mut View) + 'static,
{
    fn view(&self, id: Id<S>, store: &Store, view: &mut View) {
        self(id, store, view)
    }
}


/// A callback that manipulates a state given a message.
pub trait ReducerFn<S: 'static>: 'static {
    fn reduce(&self, state_id: Id<S>, store: &mut Store, message: DynMessage);
}

impl<S, F> ReducerFn<S> for F
where
    S: State,
    F: Fn(Id<S>, &mut Store, DynMessage) + 'static,
{
    fn reduce(&self, id: Id<S>, store: &mut Store, message: DynMessage) {
        self(id, store, message)
    }
}