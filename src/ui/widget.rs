use smallvec::SmallVec;
use vello::Scene;

use crate::{FontDB, WidgetId, NodeTree};
use crate::layout::{Style, Layout};
use crate::geom::Affine;

/// A paintable UI element in a [`NodeTree`].
/// For instance, a text element, a div, a button etc.
/// Wrapped in a [`Node`] when inserted in a [`NodeTree`] which grants it parent/child relationships with other [`Widget`]s in the tree.
pub trait Widget: 'static {

    /// [`Style`] used for computing layouts.
    #[allow(unused)]
    fn style(&self, style: &mut Style) {}

    /// Invoked when a [`Widget`] finishes computing its layout.
    /// Same value is passed into paint().
    #[allow(unused)]
    fn layout(&mut self, layout: &Layout) {}

    /// Paints this [`Widget`] onto a [`Scene`].
    /// Does not paint descendants.
    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {}

    /// Renders descendant [`Widget`]s.
    #[allow(unused)]
    fn render(&self, r: &mut UIRenderer) {}
}

/// Builds the descendants of a [`Widget`] in its [`render`](Widget::render) method using the
/// [`insert`](Self::insert), [`begin`](Self::begin) and [`end`](Self::end) methods.
/// "Rendering" in this context means building a sub-tree of UI nodes.
/// Internally, this is writing to a subtree of a [`NodeTree`].
pub struct UIRenderer<'a> {
    node_tree: &'a mut NodeTree,        // Tree being written to.
    current: WidgetId,                    // "Current" widget. Calls to insert() will append children to this widget.
    last: Option<WidgetId>,               // "Last" widget inserted as a child of the "current" widget.
    ancestors: SmallVec<[WidgetId; 8]>,   // Stack of ancestors to the "current" widget. Can include parent, grandparent, etc. If empty, calls to end() will panic.
    font_db: &'a FontDB,
}

impl<'a> UIRenderer<'a> {
    pub(crate) fn new(
        node_tree: &'a mut NodeTree,
        starting_node: WidgetId,
        font_db: &'a FontDB,
    ) -> Self {
        Self {
            node_tree,
            current: starting_node,
            last: None,
            ancestors: SmallVec::new(),
            font_db,
        }
    }

    /// Inserts a widget node as a child of the "current" node.
    /// The inserted widget is considered the "last" node.
    pub fn insert(&mut self, widget: impl Widget) -> WidgetId {
        let node_id = self.node_tree.insert(widget, self.current).unwrap();
        self.last = Some(node_id);
        node_id
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
        self.last = None;
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
}

/// DSL function that just calls [`begin`](UIRenderer::begin)
#[inline(always)]
pub fn begin(r: &mut UIRenderer) {
    r.begin();
}

/// DSL function that just calls [`end`](UIRenderer::end)
#[inline(always)]
pub fn end(r: &mut UIRenderer) {
    r.end();
}
