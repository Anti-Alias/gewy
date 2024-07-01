use std::marker::PhantomData;

use taffy::Style;
use crate::{root_style, DynMessage, Id, Message, State, Store, ToUiString, UiString, UntypedId, View, Widget};


/// An inline [`Widget`] not bound to any state.
/// Descendants are generated immediately after insertion.
/// Useful as a "root widget" in an application.
pub struct Wid<V: ViewFn> {
    pub name: UiString,
    pub style: Style,
    pub view: V,
}

impl<V: ViewFn> Wid<V> {

    pub fn new(view: V) -> Self {
        Self {
            name: "wid".into(),
            style: Style::DEFAULT,
            view
        }
    }

    pub fn root(view: V) -> Self {
        Self {
            name: "root".into(),
            style: root_style(),
            view
        }
    }

    pub fn with_name(mut self, name: impl ToUiString) -> Self {
        self.name = name.to_ui_string();
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_root_style(mut self) -> Self {
        self.style = root_style();
        self
    }
}

impl<V: ViewFn> Widget for Wid<V> {

    fn name(&self) -> &str { &self.name }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    #[allow(unused)]
    fn view(&self, _store: &Store, v: &mut View) {
        let view_fn = &self.view;
        view_fn.view(v);
    }
}

/// A "component" is an inline [`Widget`] bound to some state.
/// Its descendants are populated using a view function which takes its state as an argument.
/// Descendants are generated immediately after insertion.
/// Descendants are regenerated whenever the state changes.
/// Useful as a "root widget" in an application.
pub struct Comp<S, M, U, V>
where
    S: State,
    M: Message,
    U: UpdateFn<S, M>,
    V: StateViewFn<S>,
{
    name: UiString,
    style: Style,
    state_id: Id<S>,
    update: U,
    view: V,
    phantom: PhantomData<M>,
}

impl<S, M, U, V> Comp<S, M, U, V>
where
    S: State,
    M: Message,
    U: UpdateFn<S, M>,
    V: StateViewFn<S>,
{
    /// Creates a stateful [`Widget`] implementation using a [`State`], an update function and a view function.
    pub fn new(
        state_id: Id<S>,
        update_fn: U,
        view_fn: V
    ) -> Self {
        Self {
            name: "comp".into(),
            style: Style::DEFAULT,
            state_id,
            update: update_fn,
            view: view_fn,
            phantom: PhantomData,
        }
    }

    pub fn root(
        state: Id<S>,
        update: U,
        view: V
    ) -> Self {
        Self {
            name: "root".into(),
            style: root_style(),
            state_id: state,
            update,
            view,
            phantom: PhantomData,
        }
    }

    pub fn with_name(mut self, name: impl ToUiString) -> Self {
        self.name = name.to_ui_string();
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_root_style(mut self) -> Self {
        self.style = root_style();
        self
    }
}


impl<S, M, U, V> Widget for Comp<S, M, U, V>
where
    S: State,
    M: Message,
    U: UpdateFn<S, M>,
    V: StateViewFn<S>,
{

    fn name(&self) -> &str { &self.name }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    fn state_id(&self) -> Option<&UntypedId> {
        Some(&self.state_id.untyped())
    }

    fn update(&self, state_id: UntypedId, store: &mut Store, message: DynMessage) {
        let state_id: Id<S> = Id::from(state_id);
        if let Some(msg) = message.downcast_ref() {
            let update = &self.update;
            let params = UpdateParams { state_id, msg, store };
            update.update(params);
        }
    }

    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        let view = &self.view;
        view.view(self.state_id.clone_weak(), store, v);
    }
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

/// A function that builds the descendants of a [`Widget`] with respect to some state.
pub trait StateViewFn<S: State>: 'static {
    fn view(&self, state_id: Id<S>, store: &Store, view: &mut View);
}

impl<S: State, F> StateViewFn<S> for F
where
    F: Fn(ViewParams<S>) + 'static,
{
    fn view(&self, state_id: Id<S>, store: &Store, view: &mut View) {
        let params = ViewParams { state_id, store, view };
        self(params)
    }
}


/// A function that manipulates a state "S" given a message "M".
pub trait UpdateFn<S, M>: 'static
where
    S: State,
    M: Message,
{
    fn update(&self, params: UpdateParams<S, M>);
}

impl<S, M, F> UpdateFn<S, M> for F
where
    S: State,
    M: Message,
    F: Fn(UpdateParams<S, M>) + 'static,
{
    fn update(&self, params: UpdateParams<S, M>) {
        self(params)
    }
}


/// Parameters sent to a [`StateViewFn`].
pub struct ViewParams<'a, 'b, S: State> {
    pub state_id: Id<S>,
    pub store: &'a Store,
    pub view: &'a mut View<'b>,
}

impl<'a, 'b, S: State> ViewParams<'a, 'b, S> {
    pub fn state(&self) -> &S {
        self.store.get(&self.state_id)
    }
    pub fn state_view(&mut self) -> (&S, &mut View<'b>) {
        let state = self.store.get(&self.state_id);
        (state, self.view)
    }
}



/// Parameters sent to an [`UpdateFn`].
pub struct UpdateParams<'a, S, M>
where
    S: State,
    M: Message,
{
    pub state_id: Id<S>,
    pub msg: &'a M,
    pub store: &'a mut Store,
}

impl<'a, S, M> UpdateParams<'a, S, M>
where
    S: State,
    M: Message,
{
    #[inline(always)]
    pub fn state(&self) -> &S {
        self.store.get(&self.state_id)
    }

    #[inline(always)]
    pub fn state_mut(&mut self) -> &mut S {
        self.store.get_mut(&self.state_id)
    }
}