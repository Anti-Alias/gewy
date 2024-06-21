use taffy::{AvailableSpace, Size, TaffyTree};
use vello::kurbo::{Affine, Vec2};
use crate::{FontDB, Scene, Renderer, Widget};
use crate::taffy::Style;


/// ID of a [`Widget`](crate::Widget).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WidgetId(taffy::NodeId);


/// A scene graph of [`Widget`]s.
/// Every inserted [`Widget`] is wrapped in a [`Node`] which parent/child metadata.
pub struct NodeTree {
    root_id: WidgetId,
    widgets: TaffyTree<Box<dyn Widget>>,
}

impl NodeTree {

    pub fn new(root_widget: impl Widget + 'static) -> Self {
        let root_widget_style = style_of(&root_widget);
        let root_widget: Box<dyn Widget> = Box::new(root_widget);
        let mut widgets = TaffyTree::new();
        let root_id = widgets.new_leaf_with_context(root_widget_style, root_widget).unwrap();
        let root_id = WidgetId(root_id);
        Self { root_id, widgets }
    }

    pub fn root_id(&self) -> WidgetId { self.root_id }

    pub fn get(&self, id: WidgetId) -> Option<&dyn Widget> {
        self.widgets
            .get_node_context(id.0)
            .map(|widget| widget.as_ref())
    }

    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut dyn Widget> {
        self.widgets
            .get_node_context_mut(id.0)
            .map(|widget| widget.as_mut())
    }

    pub fn insert(&mut self, widget: impl Widget, parent_id: WidgetId) -> Option<WidgetId> {
        let widget_style = style_of(&widget);
        let widget_id = self.widgets.new_leaf_with_context(widget_style, Box::new(widget)).unwrap();
        let widget_id = WidgetId(widget_id);
        if let Err(_) = self.widgets.add_child(parent_id.0, widget_id.0) {
            self.widgets.remove(widget_id.0).unwrap();
            return None;
        }
        Some(widget_id)
    }

    /// Recursively removes the specified node, and all of its descendants.
    /// Returns true if node was in fact removed.
    pub fn remove(&mut self, id: WidgetId) -> bool {
        match self.widgets.remove(id.0) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Recursively removes the children of a [`Widget`], but not the [`Widget`] itself.
    /// This is generally used when "rerendering" a [`Widget`].
    pub fn remove_children(&mut self, id: WidgetId) {
        let Ok(children_ids) = self.widgets.children(id.0) else { return };
        for child_id in children_ids {
            self.widgets.remove(child_id).unwrap();
        }
    }

    pub fn len(&self) -> usize {
        self.widgets.total_node_count()
    }

    /// Clears the descendants of a [`Widget`] (if any), then renders them.
    pub(crate) fn render(&mut self, id: WidgetId, font_db: &FontDB) {
        self.remove_children(id);
        let Some(widget) = self.widgets.get_node_context(id.0) else { return };
        let widget: &dyn Widget = unsafe {
            let widget = widget.as_ref();
            std::mem::transmute(widget)
        };
        let mut renderer = Renderer::new(self, id, font_db);
        widget.render(&mut renderer);
    }

    pub(crate) fn paint_root(&self, scene: &mut Scene) {
        self.paint(self.root_id, scene, Affine::IDENTITY);
    }

    pub(crate) fn paint(&self, id: WidgetId, scene: &mut Scene, mut affine: Affine) {
        // Paints widget
        let widget = self.widgets.get_node_context(id.0).unwrap();
        let widget_layout = self.widgets.layout(id.0).unwrap();
        widget.paint(scene, widget_layout, affine);


        // Paints children
        let widget_children = self.widgets.children(id.0).unwrap();
        if widget_children.is_empty() { return };
        let transl = Vec2::new(widget_layout.location.x as f64, widget_layout.location.y as f64);

        affine = affine.then_translate(transl);
        for child_id in widget_children {
            self.paint(WidgetId(child_id), scene, affine);
        }
    }

    pub(crate) fn compute_layout_root(&mut self, width: f32, height: f32) {
        self.compute_layout(self.root_id, width, height);
    }

    /// Computes the layout of the [`Widget`] specified recursively.
    pub(crate) fn compute_layout(&mut self, id: WidgetId, width: f32, height: f32) {
        let space = Size {
            width: AvailableSpace::Definite(width),
            height: AvailableSpace::Definite(height),
        };
        self.widgets.compute_layout_with_measure(id.0, space, |size, size_available, _, widget, _| {
            let widget = widget.unwrap();
            let measured_size = widget.measure(size, size_available);
            measured_size
        }).unwrap();
    }
}

fn style_of(widget: &dyn Widget) -> Style {
    let mut result = Style::default();
    widget.style(&mut result);
    result
}

#[cfg(test)]
mod test {
    use crate::{NodeTree, Widget};

    struct BlankWidget;
    impl Widget for BlankWidget {}


    //////////////////////////////
    //          A
    //         / \
    //        B   D
    //       /   / \
    //      C   E   F
    #[test]
    fn test_len() {
        let mut tree = NodeTree::new(BlankWidget);

        // Builds tree
        let a_id = tree.root_id;
        let b_id = tree.insert(BlankWidget, a_id).unwrap();
        let _c_id = tree.insert(BlankWidget, b_id).unwrap();
        let d_id = tree.insert(BlankWidget, a_id).unwrap();
        let e_id = tree.insert(BlankWidget, d_id).unwrap();
        let f_id = tree.insert(BlankWidget, d_id).unwrap();

        // Removes elements and checks length
        assert_eq!(6, tree.len());
        tree.remove(e_id);
        assert_eq!(5, tree.len());
        tree.remove(f_id);
        assert_eq!(4, tree.len());
        tree.remove(b_id);
        assert_eq!(2, tree.len());
    }
}