use slotmap::SlotMap;
use smallvec::SmallVec;
use taffy::{AvailableSpace, Layout, Size, TaffyTree};
use crate::{UIRenderer, Widget, Scene};


/**
 * A scene graph of [`Node`]s containing [`Widget`]s.
 * Every inserted [`Widget`] is wrapped in a node which contains parent-child information
 * useful for navigating through the tree.
 * This tree structure 
 */
pub struct NodeTree {
    root_id: NodeId,
    nodes: SlotMap<NodeId, Node>,
    taffy_tree: TaffyTree,          // Mirrors the tree. Used for layout.
}

impl NodeTree {

    pub fn new(root_widget: impl Widget) -> Self {
        let mut nodes = SlotMap::default();
        let mut taffy_tree = TaffyTree::new();
        let taffy_root_id = taffy_tree.new_leaf(root_widget.style()).unwrap();
        let root_id = nodes.insert(Node::root(root_widget, taffy_root_id));
        Self {
            root_id,
            nodes,
            taffy_tree,
        }
    }

    pub fn root_id(&self) -> NodeId { self.root_id }

    pub fn get(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    pub fn insert(&mut self, widget: impl Widget, parent_id: NodeId) -> Option<NodeId> {
        let taffy_node_id = self.taffy_tree.new_leaf(widget.style()).unwrap();
        let node_id = self.nodes.insert(Node::new(widget, parent_id, taffy_node_id));
        let Some(parent) = self.nodes.get_mut(parent_id) else {
            self.nodes.remove(node_id);
            self.taffy_tree.remove(taffy_node_id).unwrap();
            return None;
        };
        self.taffy_tree.add_child(parent.taffy_node_id, taffy_node_id).unwrap();
        parent.children_ids.push(node_id);
        Some(node_id)
    }

    /// Recursively removes the specified node, and all of its descendants.
    /// Returns true if node was in fact removed.
    pub fn remove(&mut self, node_id: NodeId) -> bool {
        if node_id == self.root_id { panic!("Cannot remove root node") }
        let Some(node) = self.nodes.remove(node_id) else { return false };
        self.taffy_tree.remove(node.taffy_node_id).unwrap();
        let parent = self.nodes.get_mut(node.parent_id.unwrap()).unwrap();
        parent.children_ids.retain(|child_id| *child_id != node_id);
        for child_id in node.children_ids {
            self.remove_child(child_id);
        }
        true
    }

    /// Recursively removes the children of a [`Widget`], but not the [`Widget`] itself.
    /// This is generally used when "rerendering" a [`Widget`].
    pub fn remove_children(&mut self, node_id: NodeId) {
        let Some(node) = self.nodes.get_mut(node_id) else { return };
        let children = std::mem::take(&mut node.children_ids);
        for child_id in children {
            let child = self.get(child_id).unwrap();
            self.taffy_tree.remove(child.taffy_node_id).unwrap();
            self.remove_child(child_id);
        }
    }

    // Internal recursive removal function.
    // Does not delink from parent as it assumes its parent, if any, has already been removed.
    fn remove_child(&mut self, node_id: NodeId) {
        let Some(node) = self.nodes.remove(node_id) else { return };
        self.taffy_tree.remove(node.taffy_node_id).unwrap();
        for child_id in node.children_ids {
            self.remove_child(child_id);
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Clears the descendants of a [`Widget`] (if any), then renders them.
    pub(crate) fn render(&mut self, node_id: NodeId) {
        self.remove_children(node_id);
        let Some(node) = self.nodes.get_mut(node_id) else { return };
        let node: &mut Node = unsafe {
            std::mem::transmute(node)   // Safety: Render method only has access to descendants of this node.
        };
        let mut renderer = UIRenderer::new(self, node_id);
        node.widget.render(&mut renderer);
    }

    pub(crate) fn paint_root(&self, scene: &mut Scene) {
        self.paint(self.root_id, scene);
    }

    pub(crate) fn paint(&self, node_id: NodeId, scene: &mut Scene) {
        let node = self.nodes.get(node_id).unwrap();
        let node_layout = self.taffy_tree.layout(node.taffy_node_id).unwrap();
        node.widget.paint(scene, node_layout);
        for child_id in &node.children_ids {
            self.paint(*child_id, scene);
        }
    }

    pub(crate) fn compute_root_layout(&mut self, width: f32, height: f32) {
        let Some(node) = self.nodes.get(self.root_id) else { return };
        let space = Size {
            width: AvailableSpace::Definite(width),
            height: AvailableSpace::Definite(height),
        };
        self.taffy_tree.compute_layout(node.taffy_node_id, space).unwrap();
    }

    pub(crate) fn compute_layout_root(&mut self, width: f32, height: f32) {
        self.compute_layout(self.root_id, width, height);
    }

    pub(crate) fn compute_layout(&mut self, node_id: NodeId, width: f32, height: f32) {
        let Some(node) = self.nodes.get(node_id) else { return };
        let space = Size {
            width: AvailableSpace::Definite(width),
            height: AvailableSpace::Definite(height),
        };
        self.taffy_tree.compute_layout(node.taffy_node_id, space).unwrap();
    }

    pub(crate) fn layout(&self, node_id: NodeId) -> Option<&Layout> {
        self.nodes
            .get(node_id)
            .map(|node| self.taffy_tree.layout(node.taffy_node_id).unwrap())
    }
}


/// A container for a [`Widget`] residing in a [`NodeTree`].
/// Grants it a parent/child relationship with other [`Widget`]s in the same tree.
pub struct Node {
    widget: Box<dyn Widget>,
    parent_id: Option<NodeId>,
    children_ids: SmallVec<[NodeId; 8]>,
    taffy_node_id: taffy::NodeId,
}

impl Node {

    pub(crate) fn root(widget: impl Widget, taffy_node_id: taffy::NodeId) -> Self {
        Self {
            widget: Box::new(widget),
            parent_id: None,
            children_ids: SmallVec::default(),
            taffy_node_id,
        }
    }

    pub(crate) fn new(widget: impl Widget, parent_id: NodeId, taffy_node_id: taffy::NodeId) -> Self {
        Self {
            widget: Box::new(widget),
            parent_id: Some(parent_id),
            children_ids: SmallVec::default(),
            taffy_node_id,
        }
    }

    /// ID of the parent. [`None`](Option::None) if root.
    pub fn parent_id(&self) -> Option<NodeId> {
        self.parent_id
    }

    /// IDs of its children.
    pub fn children_ids(&self) -> &[NodeId] {
        &self.children_ids
    }
}

slotmap::new_key_type! {
    pub struct NodeId;
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
    fn test_hierarchy() {
        let mut tree = NodeTree::new(BlankWidget);

        // Builds tree
        let a_id = tree.root_id;
        let b_id = tree.insert(BlankWidget, a_id).unwrap();
        let c_id = tree.insert(BlankWidget, b_id).unwrap();
        let d_id = tree.insert(BlankWidget, a_id).unwrap();
        let e_id = tree.insert(BlankWidget, d_id).unwrap();
        let f_id = tree.insert(BlankWidget, d_id).unwrap();
        let a = tree.get(a_id).unwrap();
        let b = tree.get(b_id).unwrap();
        let c = tree.get(c_id).unwrap();
        let d = tree.get(d_id).unwrap();
        let e = tree.get(e_id).unwrap();
        let f = tree.get(f_id).unwrap();

        // Tests structure
        assert_eq!(None, a.parent_id);
        assert_eq!(Some(a_id), b.parent_id);
        assert_eq!(Some(a_id), d.parent_id);
        assert_eq!(&[b_id, d_id], a.children_ids());
        assert!(c.children_ids().is_empty());
        assert_eq!(Some(d_id), e.parent_id);
        assert_eq!(Some(d_id), f.parent_id);
        assert!(e.children_ids().is_empty());
        assert!(f.children_ids().is_empty());
    }

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