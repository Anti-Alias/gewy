use smallvec::SmallVec;
use crate::{Comp, Component, Widget, WidgetId, UI};

/// A list of instructions that will build a subtree of [`Widget`]s.
pub struct View {
    commands: SmallVec<[ViewCommand; 32]>,
    depth: u32,
}
impl View {

    pub fn new() -> Self {
        Self {
            commands: SmallVec::new(),
            depth: 0,
        }
    }

    /// Inserts a [`Widget`] as the child of the current [`Widget`].
    #[inline(always)]
    pub fn insert(&mut self, widget: impl Widget) {
        let command = ViewCommand::Insert(Box::new(widget));
        self.commands.push(command);
    }

    /// Inserts a [`Component`].
    pub fn insert_component<C: Component>(&mut self, component: C) {
        let widget = Comp(component);
        let command = ViewCommand::Insert(Box::new(widget));
        self.commands.push(command);
    }

    /// Inserts a [`Widget`] as the child of the current [`Widget`].
    /// Subsequent insertions will be children of this [`Widget`].
    #[inline(always)]
    pub fn begin(&mut self, widget: impl Widget) {
        let command = ViewCommand::Begin(Box::new(widget));
        self.commands.push(command);
        self.depth += 1;
    }

    /// Causes subsequent insertions to be children of the last widget inserted.
    #[inline(always)]
    pub fn end(&mut self) {
        if self.depth == 0 {
            panic!("Cannot end here");
        }
        self.commands.push(ViewCommand::End);
        self.depth -= 1;
    }

    /// Executes all commands in the list and consumes self.
    pub(crate) fn execute(self, mut widget_id: WidgetId, ui: &mut UI) {
        for command in self.commands {
            match command {
                ViewCommand::Insert(widget) => {
                    ui.insert_box(widget, widget_id).unwrap();
                },
                ViewCommand::Begin(widget) => {
                    let child_id = ui.insert_box(widget, widget_id).unwrap();
                    widget_id = child_id;
                },
                ViewCommand::End => {
                    widget_id = ui.parent_id_of(widget_id).unwrap();
                },
            }
        }
    }
}

/// A command in a [`ViewCmds`] object.
/// Used to build a UI tree.
enum ViewCommand {
    Insert(Box<dyn Widget>),
    Begin(Box<dyn Widget>),
    End,
}
