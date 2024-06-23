use std::any::Any;

use crate::{Class, GewyString, RawId, Id, Store, View, Widget, WidgetId};
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
    S: 'static,
    V: StateViewFn<S>,
{
    pub style: Style,
    pub state: Id<S>,
    pub view_fn: V,
}

impl<S, V> Widget for Comp<S, V>
where
    S: 'static,
    V: StateViewFn<S>,
{

    fn name(&self) -> GewyString {
        "comp".into()
    }

    fn style(&self, style: &mut Style) {
        *style = self.style.clone();
    }

    fn state(&self) -> Option<RawId> {
        Some(self.state.handle)
    }

    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        let state_value = store.get(&self.state);
        let view_fn = &self.view_fn;
        view_fn.view(self.state.clone(), state_value, v);
    }
}

impl<S, V> Comp<S, V>
where
    S: 'static,
    V: StateViewFn<S>,
{
    pub fn new(state: Id<S>, class: impl Class<Style>, view_fn: V) -> Self {
        Self {
            style: class.produce(),
            state,
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
pub trait StateViewFn<S: Any>: 'static {
    fn view(&self, state: Id<S>, state_value: &S, view: &mut View);
}

impl<S: Any, F> StateViewFn<S> for F
where
    F: Fn(Id<S>, &S, &mut View) + 'static,
{
    fn view(&self, state: Id<S>, state_value: &S, view: &mut View) {
        self(state, state_value, view)
    }
}


/// Insertion function for a [`Component`].
pub fn comp<S, V>(
    state: Id<S>,
    class: impl Class<Style>,
    view: &mut View,
    view_fn: V,
) -> WidgetId
where
    S: 'static,
    V: StateViewFn<S>,
{
    view.insert(Comp {
        style: class.produce(),
        state,
        view_fn,
    })
}