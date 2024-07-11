use std::collections::HashMap;
use std::ops::DerefMut;
use taffy::{AvailableSpace, Size, TaffyTree};
use vello::kurbo::{Affine, Vec2};
use crate::{DynMessage, FontDB, InputMessage, MouseButton, RawId, Store, Widget};
use crate::vello::Scene;
use crate::taffy::Style;

/// A user interface represented as a tree of [`Widget`]s.
pub struct UI {
    root_id: WidgetId,
    widgets: TaffyTree<Box<dyn Widget>>,
    state_bindings: HashMap<RawId, Vec<WidgetId>>,
    to_render: Vec<WidgetId>,
    cursor: Cursor,
}

impl UI {

    /// Creates a UI with the specified [`Widget`] as its root.
    pub fn new(widget: impl Widget + 'static) -> Self {
        let mut widgets: TaffyTree<Box<dyn Widget>> = TaffyTree::new();
        let style = style_of(&widget);
        let is_stateful = widget.state_id().is_some();
        let id = widgets.new_leaf_with_context(style, Box::new(widget)).unwrap();
        let id = WidgetId(id);
        let mut to_render = vec![];
        if is_stateful {
            to_render.push(id);
        }
        Self {
            root_id: id,
            widgets,
            state_bindings: HashMap::new(),
            to_render,
            cursor: Cursor::default(),
        }
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
        self.insert_box(Box::new(widget), parent_id)
    }

    pub(crate) fn insert_box(&mut self, widget: Box<dyn Widget>, parent_id: WidgetId) -> Option<WidgetId> {
        // Inserts widget
        let widget_state = widget.state_id().map(|id| id.raw());
        let mut widget_style = Style::default();
        widget.style(&mut widget_style);
        let widget_id = self.widgets.new_leaf_with_context(widget_style, widget).unwrap();
        let widget_id = WidgetId(widget_id);
        if let Err(_) = self.widgets.add_child(parent_id.0, widget_id.0) {
            self.widgets.remove(widget_id.0).unwrap();
            return None;
        }
        // Binds state and schedules rendering for that widget
        if let Some(widget_state) = widget_state {
            self.bind_state(widget_id, widget_state);
            self.to_render.push(widget_id);
        }
        Some(widget_id)
    }

    pub(crate) fn parent_id_of(&self, widget_id: WidgetId) -> Option<WidgetId> {
        self.widgets
            .parent(widget_id.0)
            .map(|parent_id| WidgetId(parent_id))
    }

    /// Recursively initializes widgets
    pub(crate) fn init(&mut self, widget_id: WidgetId, fonts: &FontDB) {
        let mut init = move |widget: &mut dyn Widget| widget.init(fonts);
        self.update(widget_id, &mut init);
    }

    pub fn bind_state(&mut self, widget_id: WidgetId, state_id: RawId) {
        let state_binding = self.state_bindings.entry(state_id).or_default();
        state_binding.push(widget_id);
    }

    pub fn unbind_state(&mut self, widget_id: WidgetId, state_id: RawId) {
        let Some(state_binding) = self.state_bindings.get_mut(&state_id) else { return };
        state_binding.retain(|wid| *wid != widget_id);
        if state_binding.is_empty() {
            self.state_bindings.remove(&state_id);
        }
    }

    /// Recursively removes the specified node, and all of its descendants.
    /// Returns true if node was in fact removed.
    pub fn remove(&mut self, id: WidgetId) -> bool {
        if !self.contains(id) { return false }
        let children_ids = self.widgets.children(id.0).unwrap();
        for child_id in children_ids {
            self.remove(WidgetId(child_id));
        }
        self.widgets.set_node_context(id.0, None).unwrap();
        self.widgets.remove(id.0).unwrap();
        true
    }

    pub fn len(&self) -> usize {
        self.widgets.total_node_count()
    }

    /// Rerenders [`Widget`]s bound the states provided.
    /// Returns true if layout / repainting are necessary.
    pub fn inform_state_changes(&mut self, state_ids: &[RawId]) -> usize {
        for state_id in state_ids {
            let Some(binding) = self.state_bindings.get_mut(state_id) else { continue };
            self.to_render.extend(binding.iter().copied());
        }
        self.to_render.len()
    }

    pub fn fire_input_message(&mut self, msg: InputMessage, store: &mut Store) {
        match msg {
            InputMessage::CursorEntered  =>  {
                self.cursor.on_screen = true;
            },
            InputMessage::CursorLeft => {
                self.cursor.on_screen = false;
            },
            InputMessage::CursorMoved { x, y } => {
                self.cursor.location = Location { x, y };
            },
            InputMessage::MousePressed { button: MouseButton::Left }  => {
                self.cursor.pressed_location = Some(self.cursor.location);
                let Some(widget_id) = self.widget_touching(self.root_id, self.cursor.location) else { return };
                let msg = DynMessage::new(msg);
                self.bubble_message(msg, widget_id, store);
            },
            InputMessage::MouseReleased { button: MouseButton::Left } => {
                let pressed_loc = self.cursor.pressed_location.take().unwrap();
                let released_loc = self.cursor.location;
                let Some(pressed_widget_id) = self.widget_touching(self.root_id, pressed_loc) else { return };
                let Some(released_widget_id) = self.widget_touching(self.root_id, released_loc) else { return };
                if pressed_widget_id == released_widget_id {
                    let msg = DynMessage::new(msg);
                    self.bubble_message(msg, released_widget_id, store);
                }
            }
            _ => {}
        }
    }

    fn widget_touching(&self, widget_id: WidgetId, loc: Location) -> Option<WidgetId> {
        let widget_layout = self.widgets.layout(widget_id.0).unwrap();
        let loc = Location {
            x: loc.x - widget_layout.location.x,
            y: loc.y - widget_layout.location.y,
        };
        for child_id in self.widgets.children(widget_id.0).unwrap() {
            let child_id = WidgetId(child_id);
            if let Some(descendant_id) = self.widget_touching(child_id, loc) {
                return Some(descendant_id);
            }
        }
        let widget = self.widgets.get_node_context(widget_id.0).unwrap();
        match widget.touches(loc.x, loc.y, widget_layout.size.width, widget_layout.size.height) {
            true => Some(widget_id),
            false => None,
        }
    }

    fn bubble_message(&mut self, mut msg: DynMessage, mut widget_id: WidgetId, store: &mut Store) {
        loop {
            let widget = self.widgets.get_node_context_mut(widget_id.0).unwrap();
            let Some(new_msg) = widget.update(store, msg) else { return };
            let Some(parent_id) = self.widgets.parent(widget_id.0) else { return };
            msg = new_msg;
            widget_id = WidgetId(parent_id);
        }
    }

    pub fn render(&mut self, fonts: &FontDB, store: &Store) {
        while !self.to_render.is_empty() {
            for id in std::mem::take(&mut self.to_render) {
                self.render_at(id, fonts, store);
            }
        }
    }

    pub fn contains(&self, id: WidgetId) -> bool {
        self.widgets.get_node_context(id.0).is_some()
    }

    /// Clears the descendants of a [`Widget`] (if any), then renders them.
    pub(crate) fn render_at(&mut self, widget_id: WidgetId, fonts: &FontDB, store: &Store) {
        if !self.contains(widget_id) { return }
        let children_ids = self.widgets.children(widget_id.0).unwrap();
        for child_id in children_ids {
            let child_id = WidgetId(child_id);
            self.remove(child_id);
        }
        let Some(widget) = self.widgets.get_node_context(widget_id.0) else { return };
        let widget: &dyn Widget = unsafe {
            let widget = widget.as_ref();
            std::mem::transmute(widget)
        };
        let view_commands = widget.view(store);
        view_commands.execute(widget_id, self);
        self.init(widget_id, fonts);
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

    pub(crate) fn compute_layout(&mut self, width: f32, height: f32) {
        self.compute_layout_at(self.root_id, width, height);
    }

    /// Computes the layout of the [`Widget`] specified recursively.
    pub(crate) fn compute_layout_at(&mut self, id: WidgetId, width: f32, height: f32) {
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

    fn update<F>(&mut self, widget_id: WidgetId, callback: &mut F)
    where
        F: FnMut(&mut dyn Widget)
    {
        let Some(widget) = self.widgets.get_node_context_mut(widget_id.0) else { return };
        callback(widget.deref_mut());

        for child_id in self.widgets.children(widget_id.0).unwrap() {
            let child_id = WidgetId(child_id);
            self.update(child_id, callback);
        }
    }
}

pub fn remove_children(
    widgets: &mut TaffyTree<Box<dyn Widget>>,
    id: WidgetId
) {
    let Ok(children_ids) = widgets.children(id.0) else { return };
    for child_id in children_ids {
        widgets.remove(child_id).unwrap();
    }
}

fn style_of(widget: &dyn Widget) -> Style {
    let mut result = Style::default();
    widget.style(&mut result);
    result
}


/// Id of a [`Widget`](crate::Widget) an a [`UI`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WidgetId(taffy::NodeId);

impl WidgetId {
    pub fn set(self, id: &mut WidgetId) {
        *id = self;
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self(taffy::NodeId::new(u64::MAX))
    }
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
struct Location { x: f32, y: f32 }

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Cursor {
    location: Location,
    pressed_location: Option<Location>,
    on_screen: bool,
}

#[cfg(test)]
mod test {
    use crate::{UI, Widget};

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
        let mut tree = UI::new(BlankWidget);

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