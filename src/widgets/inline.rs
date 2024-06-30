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
struct Comp<S, U, V>
where
    S: State,
    U: UpdateFn<S>,
    V: StateViewFn<S>,
{
    pub state_handle: Handle<S>,
    pub update: U,
    pub view: V,
}

impl<S, R, V> Widget for Comp<S, R, V>
where
    S: State,
    R: UpdateFn<S>,
    V: StateViewFn<S>,
{

    fn name(&self) -> GewyString {
        "comp".into()
    }

    fn state_id(&self) -> Option<RawId> {
        Some(self.state_handle.id().raw())
    }

    fn update(&self, state_id: RawId, store: &mut Store, message: DynMessage) {
        let update = &self.update;
        update.update(Id::from(state_id), store, message);
    }

    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        let view = &self.view;
        let state = store.get(&self.state_handle);
        view.view(state, store, v);
    }
}

impl<S, U, V> Comp<S, U, V>
where
    S: State,
    U: UpdateFn<S>,
    V: StateViewFn<S>,
{
    fn new(state: Handle<S>, update: U, view: V) -> Self {
        Self {
            state_handle: state,
            update,
            view,
        }
    }
}

/// Creates a component widget using a state, an update function and a view function.
pub fn create_comp<S, U, V>(
    state: Handle<S>,
    update: U,
    view: V
) -> impl Widget
where
    S: State,
    U: UpdateFn<S>,
    V: StateViewFn<S>,
{
    Comp::new(state, update, view)
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
    fn view(&self, state: &S, store: &Store, view: &mut View);
}

impl<S: State, F> StateViewFn<S> for F
where
    F: Fn(&S, &Store, &mut View) + 'static,
{
    fn view(&self, state: &S, store: &Store, view: &mut View) {
        self(state, store, view)
    }
}


/// A callback that manipulates a state given a message.
pub trait UpdateFn<S: 'static>: 'static {
    fn update(&self, state_id: Id<S>, store: &mut Store, message: DynMessage);
}

impl<S, F> UpdateFn<S> for F
where
    S: State,
    F: Fn(Id<S>, &mut Store, DynMessage) + 'static,
{
    fn update(&self, id: Id<S>, store: &mut Store, message: DynMessage) {
        self(id, store, message)
    }
}