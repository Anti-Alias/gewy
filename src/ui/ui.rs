use std::collections::HashMap;

use taffy::{AvailableSpace, Size, TaffyTree, TraversePartialTree};
use vello::kurbo::{Affine, Vec2};
use crate::{EventCtx, FontDB, InputEvent, MouseButton, RawId, Store, View, Widget, WidgetEvent};
use crate::vello::Scene;
use crate::taffy::Style;


/// ID of a [`Widget`](crate::Widget).
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


/// A scene graph of [`Widget`]s.
/// Every inserted [`Widget`] is wrapped in a [`Node`] which parent/child metadata.
pub struct UI {
    root_id: WidgetId,
    widgets: TaffyTree<Box<dyn Widget>>,
    state_bindings: HashMap<RawId, Vec<WidgetId>>,
    to_render: Vec<WidgetId>,
    cursor: Cursor,
}

impl UI {

    pub fn new(root_widget: impl Widget + 'static) -> Self {
        let root_widget_style = style_of(&root_widget);
        let root_widget: Box<dyn Widget> = Box::new(root_widget);
        let mut widgets = TaffyTree::new();
        let state_id = root_widget.state();
        let root_id = widgets.new_leaf_with_context(root_widget_style, root_widget).unwrap();
        let root_id = WidgetId(root_id);
        let mut result = Self {
            root_id,
            widgets,
            state_bindings: HashMap::new(),
            to_render: vec![root_id],
            cursor: Cursor::default(),
        };
        if let Some(state_id) = state_id {
            result.bind_state(root_id, state_id)
        }
        result
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
        let state_id = widget.state();
        let disable_view = widget.disable_view();
        let widget_style = style_of(&widget);
        let widget_id = self.widgets.new_leaf_with_context(widget_style, Box::new(widget)).unwrap();
        let widget_id = WidgetId(widget_id);
        if let Err(_) = self.widgets.add_child(parent_id.0, widget_id.0) {
            self.widgets.remove(widget_id.0).unwrap();
            return None;
        }
        if let Some(state_id) = state_id {
            self.bind_state(widget_id, state_id);
        }
        if !disable_view {
            self.to_render.push(widget_id);
        }
        Some(widget_id)
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
        let children_ids = self.widgets.children(id.0).unwrap();
        for child_id in children_ids {
            self.remove(WidgetId(child_id));
        }
        match self.widgets.remove(id.0) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Recursively removes the children of a [`Widget`], but not the [`Widget`] itself.
    /// This is generally used when "rerendering" a [`Widget`].
    pub fn remove_children(&mut self, id: WidgetId) {
        let children_ids = self.widgets.children(id.0).unwrap();
        for child_id in children_ids {
            self.remove(WidgetId(child_id));
        }
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

    pub fn fire_input_event(&mut self, event: InputEvent, store: &mut Store) {
        match event {
            InputEvent::MousePressed { button } => self.bubble(move |widget, x, y, width, height| {
                let ctx = EventCtx { store };
                let event = WidgetEvent::Pressed { mouse_button: button, mouse_x: x, mouse_y: y, width, height };
                widget.event(event, ctx)
            }),
            InputEvent::MouseReleased { button } => self.bubble(|widget, x, y, width, height| {
                let ctx = EventCtx { store };
                let event = WidgetEvent::Released { mouse_button: button, mouse_x: x, mouse_y: y, width, height };
                widget.event(event, ctx)
            }),
            InputEvent::CursorEntered               => self.cursor.on_screen = true,
            InputEvent::CursorLeft                  => self.cursor.on_screen = false,
            InputEvent::CursorMoved { x, y }        => { self.cursor.x = x; self.cursor.y = y },
        }
    }

    fn bubble(&self, mut callback: impl FnMut(&dyn Widget, f32, f32, f32, f32) -> bool) {
        self.bubble_at(self.root_id, self.cursor.x, self.cursor.y, &mut callback);
    }

    fn bubble_at(
        &self,
        widget_id: WidgetId,
        x: f32,
        y: f32,
        callback: &mut impl FnMut(&dyn Widget, f32, f32, f32, f32) -> bool,
    ) -> bool {
        let widget_layout = self.widgets.layout(widget_id.0).unwrap();
        let x = x - widget_layout.location.x;
        let y = y - widget_layout.location.y;
        for child_id in self.widgets.children(widget_id.0).unwrap() {
            let child_id = WidgetId(child_id);
            let propagate_event = self.bubble_at(child_id, x, y, callback);
            if !propagate_event { return false }
        }
        let widget = self.widgets.get_node_context(widget_id.0).unwrap();
        let widget_size = widget_layout.size;
        if x >= 0.0 && y >= 0.0 && x <= widget_size.width && y <= widget_size.height {
            callback(widget.as_ref(), x, y, widget_size.width, widget_size.height)
        }
        else {
            true
        }
    }

    pub fn render(&mut self, font_db: &FontDB, store: &Store) {
        while !self.to_render.is_empty() {
            for id in std::mem::take(&mut self.to_render) {
                self.render_at(id, font_db, store);
            }
        }
    }

    pub fn contains(&self, id: WidgetId) -> bool {
        self.widgets.contains(id.0)
    }

    /// Clears the descendants of a [`Widget`] (if any), then renders them.
    pub(crate) fn render_at(&mut self, id: WidgetId, font_db: &FontDB, store: &Store) {
        if !self.contains(id) { return }
        self.remove_children(id);
        let Some(widget) = self.widgets.get_node_context(id.0) else { return };
        let widget: &dyn Widget = unsafe {
            let widget = widget.as_ref();
            std::mem::transmute(widget)
        };
        let mut renderer = View::new(self, id, font_db);
        widget.view(store, &mut renderer);
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

    pub fn print_diagnostics(&self) {
        self.widgets.print_diagnostics();
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

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub left_button_pressed: bool,
    pub on_screen: bool,
}

impl Cursor {
    pub fn set_button_pressed(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.left_button_pressed = pressed,
            _ => {}
        }
    }
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