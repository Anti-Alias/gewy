use smallvec::SmallVec;
use vello::Scene;

use crate::{NodeId, NodeTree};
use crate::layout::{Style, Layout};

/// A paintable UI element in a [`NodeTree`].
/// For instance, a text element, a div, a button etc.
/// Wrapped in a [`Node`] when inserted in a [`NodeTree`] which grants it parent/child relationships with other [`Widget`]s in the tree.
pub trait Widget: 'static {
    fn style(&self) -> Style { Style::default() }
    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout) {}
    #[allow(unused)]
    fn render(&self, r: &mut UIRenderer) {}
}

/// Builds the descendants of a [`Widget`] in its [`render`](Widget::render) method using the
/// [`insert`](Self::insert), [`begin`](Self::begin) and [`end`](Self::end) methods.
/// "Rendering" in this context means building a sub-tree of UI nodes.
/// Internally, this is writing to a subtree of a [`NodeTree`].
pub struct UIRenderer<'a> {
    node_tree: &'a mut NodeTree,        // Tree being written to.
    current: NodeId,                    // "Current" widget. Calls to insert() will append children to this widget.
    last: Option<NodeId>,               // "Last" widget inserted as a child of the "current" widget.
    ancestors: SmallVec<[NodeId; 8]>,   // Stack of ancestors to the "current" widget. Can include parent, grandparent, etc. If empty, calls to end() will panic.
}

impl<'a> UIRenderer<'a> {
    pub(crate) fn new(node_tree: &'a mut NodeTree, starting_node: NodeId) -> Self {
        Self {
            node_tree,
            current: starting_node,
            last: None,
            ancestors: SmallVec::new(),
        }
    }

    /// Inserts a widget node as a child of the "current" node.
    /// The inserted widget is considered the "last" node.
    pub fn insert(&mut self, widget: impl Widget) {
        let node_id = self.node_tree.insert(widget, self.current).unwrap();
        self.last = Some(node_id);
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
