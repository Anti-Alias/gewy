use smallvec::SmallVec;
use vello::Scene;
use downcast_rs::{Downcast, impl_downcast};

use crate::{DynMessage, FontDB, RawWidgetId, Store, UntypedId, WidgetId, UI};
use crate::taffy::{Style, Layout, Size, AvailableSpace};
use crate::kurbo::Affine;

/// A paintable UI element in a [`NodeTree`].
/// For instance, a text element, a div, a button etc.
/// Wrapped in a [`Node`] when inserted in a [`NodeTree`] which grants it parent/child relationships with other [`Widget`]s in the tree.
pub trait Widget: Downcast {

    /// Display name of the widget.
    fn name(&self) -> &str { "widget" }

    /// [`Style`] used for computing layouts.
    #[allow(unused)]
    fn style(&self, style: &mut Style) {}

    #[allow(unused)]
    fn measure(&mut self, known_size: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        Size::ZERO
    }

    /// True if the coordinates specified touch this [`Widget`].
    fn touches(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        x >= 0.0 && y >= 0.0 && x <= width && y <= height
    }

    /// Raw ID 
    fn state_id(&self) -> Option<&UntypedId> { None }

    /// Initializes the widget
    #[allow(unused)]
    fn init(&mut self, fonts: &FontDB) {}

    #[allow(unused)]
    fn update(&self, store: &mut Store, message: DynMessage) -> Option<DynMessage> {
        Some(message)
    }

    /// Paints this [`Widget`] onto a [`Scene`].
    /// Does not paint descendants.
    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {}

    /// Renders descendant [`Widget`]s.
    #[allow(unused)]
    fn view(&self, store: &Store, v: &mut View) {
        if self.state_id().is_none() {
            panic!("view() called on stateless widget");
        }
    }
}

impl_downcast!(Widget);


/// Utility used to build a tree of [`Widget`]s.
pub struct View<'a> {
    ui: &'a mut UI,
    current: RawWidgetId,
    last: Option<RawWidgetId>,
    ancestors: SmallVec<[RawWidgetId; 8]>,
    fonts: &'a FontDB,
}

impl<'a> View<'a> {
    pub(crate) fn new(
        ui: &'a mut UI,
        starting_node: RawWidgetId,
        fonts: &'a FontDB,
    ) -> Self {
        Self {
            ui,
            current: starting_node,
            last: None,
            ancestors: SmallVec::new(),
            fonts,
        }
    }

    /// Inserts a widget node as a child of the "current" node.
    /// The inserted widget is considered the "last" node.
    pub fn insert<W: Widget>(&mut self, widget: W) -> WidgetId<W> {
        let widget_id = self.ui.insert(widget, self.current).unwrap();
        self.last = Some(widget_id);
        WidgetId::new(widget_id)
    }

    /// Gets the id of the last [`Widget`] inserted.
    pub fn last(&mut self) -> RawWidgetId {
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
    pub fn fonts(&self) -> &FontDB {
        &self.fonts
    }

    /// Gets a [`Widget`] by id.
    pub fn widget<W: Widget>(&self, widget_id: RawWidgetId) -> &W {
        self.ui
            .get(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_ref())
            .unwrap()
    }

    /// Gets a [`Widget`] by id.
    pub fn widget_mut<W: Widget>(&mut self, widget_id: RawWidgetId) -> &mut W {
        self.ui
            .get_mut(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_mut())
            .unwrap()
    }

    /// Gets a [`Widget`] by id.
    pub fn get_widget<W: Widget>(&self, widget_id: RawWidgetId) -> Option<&W> {
        self.ui
            .get(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_ref())
    }

    /// Gets a [`Widget`] by id.
    pub fn get_widget_mut<W: Widget>(&mut self, widget_id: RawWidgetId) -> Option<&mut W> {
        self.ui
            .get_mut(widget_id)
            .and_then(|dyn_widget| dyn_widget.downcast_mut())
    }
}

#[inline(always)]
pub fn begin(v: &mut View) {
    v.begin();
}

#[inline(always)]
pub fn end(v: &mut View) {
    v.end();
}