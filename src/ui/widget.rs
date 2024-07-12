use vello::Scene;
use downcast_rs::{Downcast, impl_downcast};

use crate::{FontDB, Msg, Store, UntypedId, View};
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
    fn update<'a>(&'a self, store: &mut Store, message: Msg<'a>) -> Option<Msg<'a>> {
        Some(message)
    }

    /// Paints this [`Widget`] onto a [`Scene`].
    /// Does not paint descendants.
    #[allow(unused)]
    fn paint(&self, scene: &mut Scene, layout: &Layout, affine: Affine) {}

    /// Renders descendant [`Widget`]s.
    #[allow(unused)]
    fn view(&self, store: &Store, view: &mut View) {}
}

impl_downcast!(Widget);