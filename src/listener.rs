use crate::{EventCtx, Id, State};

/// Any type that reacts to an event on a [`Widget`].
pub trait Listener<E>: 'static {
    fn handle(&self, event: E, ctx: &mut EventCtx);
}

impl<E, F> Listener<E> for F
where
    F: Fn(E, &mut EventCtx) + 'static
{
    fn handle(&self, event: E, ctx: &mut EventCtx) {
        self(event, ctx);
    }
}

/// Creates a listener that updates the state value provided whenever any event is fired.
pub fn update<E, S>(id: Id<S>) -> impl Listener<E> + Clone
where
    E: 'static,
    S: State,
{
    move |_evt: E, ctx: &mut EventCtx| {
        ctx.store.update(&id);
    }
}

/// Creates a listener that fires when the event equals the provided event.
#[inline(always)]
pub fn for_event<E, C>(
    event: E,
    callback: C
) -> impl Listener<E>
where
    E: PartialEq + 'static,
    C: Fn(&mut EventCtx) + 'static
{
    move |evt: E, ctx: &mut EventCtx| {
        if event != evt { return }
        callback(ctx);
    }
}