use crate::{Class, GewyString, Handle, Id, RawId, State, Store, View, Widget, WidgetId};
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
pub struct Comp<S, V>
where
    S: State,
    V: StateViewFn<S>,
{
    pub style: Style,
    pub state_handle: Handle<S>,
    pub view_fn: V,
}

impl<S, V> Widget for Comp<S, V>
where
    S: State,
    V: StateViewFn<S>,
{

    fn name(&self) -> GewyString {
        "comp".into()
    }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    fn state_id(&self) -> Option<RawId> {
        Some(self.state_handle.id().raw())
    }

    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        let view_fn = &self.view_fn;
        view_fn.view(self.state_handle.id(), store, v);
    }
}

impl<S, V> Comp<S, V>
where
    S: State,
    V: StateViewFn<S>,
{
    pub fn new(state_handle: Handle<S>, class: impl Class<Style>, view_fn: V) -> Self {
        Self {
            style: class.produce(),
            state_handle,
            view_fn,
        }
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

/// Creates a [`StateViewFn`] using a callback.
#[inline(always)]
pub fn state_view_fn<P, C, S>(
    params: P,
    callback: C,
) -> impl StateViewFn<S>
where
    P: Clone + 'static,
    C: Fn(Id<S>, &Store, P, &mut View) + 'static,
    S: State,
{
    move |id: Id<S>, store: &Store, view: &mut View| {
        let params = params.clone();
        callback(id, store, params, view);
    }
}


/// Insertion function for a [`Component`].
pub fn comp<S, V>(
    state_handle: Handle<S>,
    class: impl Class<Style>,
    view: &mut View,
    view_fn: V,
) -> WidgetId<Comp<S, V>>
where
    S: State,
    V: StateViewFn<S>,
{
    view.insert(Comp {
        style: class.produce(),
        state_handle,
        view_fn,
    })
}