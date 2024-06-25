use std::any::Any;

use smallvec::SmallVec;
use vello::Scene;
use downcast_rs::{Downcast, impl_downcast};

use crate::{FontDB, FromStore, GewyString, Id, MouseButton, RawId, Store, WidgetId, UI};
use crate::taffy::{Style, Layout, Size, AvailableSpace};
use crate::kurbo::Affine;

/// A paintable UI element in a [`NodeTree`].
/// For instance, a text element, a div, a button etc.
/// Wrapped in a [`Node`] when inserted in a [`NodeTree`] which grants it parent/child relationships with other [`Widget`]s in the tree.
pub trait Widget: Downcast {

    /// Display name of the widget.
    fn name(&self) -> GewyString { "widget".into() }

    /// [`Style`] used for computing layouts.
    #[allow(unused)]
    fn style(&self, style: &mut Style) {}


    #[allow(unused)]
    fn measure(&mut self, known_size: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        Size::ZERO
    }

    #[allow(unused)]
    fn event(&self, event: WidgetEvent, ctx: EventCtx) -> bool { true }

    fn state(&self) -> Option<RawId> { None }

    /// Paints this [`Widget`] onto a [`Scene`].
    /// Does not paint descendants.
    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {}

    /// If true, [`Widget::view`] will not be called after insertion.
    fn disable_view(&self) -> bool { false }

    /// Renders descendant [`Widget`]s.
    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {}
}

impl_downcast!(Widget);


pub struct EventCtx<'a> {
    pub(crate) store: &'a mut Store,
}

impl<'a> EventCtx<'a> {
    
    #[inline(always)]
    pub fn create_state<S: Any>(&mut self, value: S) -> Id<S> {
        self.store.create(value)
    }

    #[inline(always)]
    pub fn init_state<S: Any + FromStore>(&mut self) -> Id<S> {
        self.store.init()
    }

    /// Gets read-only access to the value of a state object.
    pub fn state<S>(&self, state_id: &Id<S>) -> &S
    where
        S: Any,
    {
        self.store.get(state_id.as_ref())
    }

    /// Gets write access to the value of a state object.
    pub fn state_mut<S>(&mut self, id: &Id<S>) -> &mut S
    where
        S: Any,
    {
        self.store.get_mut(id)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum WidgetEvent {
    Pressed {
        mouse_button: MouseButton,
        mouse_x: f32,
        mouse_y: f32,
        width: f32,
        height: f32,
    },
    Released {
        mouse_button: MouseButton,
        mouse_x: f32,
        mouse_y: f32,
        width: f32,
        height: f32,
    },
}

/// Utility used to build a tree of [`Widget`]s.
pub struct View<'a> {
    ui: &'a mut UI,
    current: WidgetId,
    last: Option<WidgetId>,
    ancestors: SmallVec<[WidgetId; 8]>,
    font_db: &'a FontDB,
}

impl<'a> View<'a> {
    pub(crate) fn new(
        ui: &'a mut UI,
        starting_node: WidgetId,
        font_db: &'a FontDB,
    ) -> Self {
        Self {
            ui,
            current: starting_node,
            last: None,
            ancestors: SmallVec::new(),
            font_db,
        }
    }

    /// Inserts a widget node as a child of the "current" node.
    /// The inserted widget is considered the "last" node.
    pub fn insert(&mut self, widget: impl Widget) -> WidgetId {
        let node_id = self.ui.insert(widget, self.current).unwrap();
        self.last = Some(node_id);
        node_id
    }

    /// Gets the id of the last [`Widget`] inserted.
    pub fn last(&mut self) -> WidgetId {
        self.last.unwrap()
    }

    /// Sets the "current" [`Widget`] to the last one inserted.
    /// Subsequent insertions will be children of this [`Widget`].
    /// Analagous to a '{' in json.
    pub fn begin(&mut self) {
        let Some(last) = self.last else {
            panic!("Cannot 'begin' here");
        };
        self.ancestors.push(self.current);
        self.current = last;
    }

    /// Sets the "current" [`Widget`] to parent of the "current" [`Widget`].
    /// Subsequent insertions will be children of this [`Widget`].
    /// Analagous to a '}' in json.
    pub fn end(&mut self) {
        let Some(parent) = self.ancestors.pop() else {
            panic!("Cannot 'end' here");
        };
        self.current = parent;
        self.last = None;
    }

    /// A database of fonts to be queried during [`Widget`] construction.
    pub fn font_db(&self) -> &FontDB {
        &self.font_db
    }

    /// Gets a [`Widget`] by id.
    pub fn widget<W: Widget>(&self, widget_id: WidgetId) -> &W {
        self.ui
            .get(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_ref())
            .unwrap()
    }

    /// Gets a [`Widget`] by id.
    pub fn widget_mut<W: Widget>(&mut self, widget_id: WidgetId) -> &mut W {
        self.ui
            .get_mut(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_mut())
            .unwrap()
    }

    /// Gets a [`Widget`] by id.
    pub fn get_widget<W: Widget>(&self, widget_id: WidgetId) -> Option<&W> {
        self.ui
            .get(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_ref())
    }

    /// Gets a [`Widget`] by id.
    pub fn get_widget_mut<W: Widget>(&mut self, widget_id: WidgetId) -> Option<&mut W> {
        self.ui
            .get_mut(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_mut())
    }
}

/// DSL function that just calls [`begin`](UIRenderer::begin)
#[inline(always)]
pub fn begin(v: &mut View) {
    v.begin();
}

/// DSL function that just calls [`end`](UIRenderer::end)
#[inline(always)]
pub fn end(v: &mut View) {
    v.end();
}


/// Any type that reacts to an event on a [`Widget`].
pub trait Listener<E>: 'static {
    fn handle(&self, event: E, ctx: EventCtx);
}

impl<E, F> Listener<E> for F
where
    F: Fn(E, EventCtx) + 'static
{
    fn handle(&self, event: E, ctx: EventCtx) {
        self(event, ctx);
    }
}

/// A helper function that creates a listener from a callback function.
pub fn listener<E, S, C>(event: E, id: &Id<S>, callback: C) -> impl Listener<E>
where
    E: PartialEq + 'static,
    S: Any,
    C: Fn(&Id<S>, &mut EventCtx) + 'static,
{
    let id = id.clone();
    move |evt: E, mut ctx: EventCtx| {
        if event != evt { return }
        callback(&id, &mut ctx);
    }
}