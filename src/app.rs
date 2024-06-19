use slotmap::SlotMap;
use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crate::{FontDB, GewyWindow, GewyWindowId, GewyWindowView, Late};

/// Application that displays a gewy UI in a single winit window.
pub struct GewyApp {
    instance: Instance,
    windows: SlotMap<GewyWindowId, GewyWindow>,    // Parallel with 'windows'.
    font_db: FontDB,
    event_handler: Option<Box<dyn FnMut(GewyAppEvent, GewyContext)>>,
    started: bool,
}

impl GewyApp {
    
    pub fn new(font_db: FontDB) -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::default()),
            windows: SlotMap::default(),
            font_db,
            event_handler: None,
            started: false,
        }
    }

    /// Starts the application. Blocks until closed.
    pub fn start(mut self, event_handler: impl FnMut(GewyAppEvent, GewyContext) + 'static) {
        self.event_handler = Some(Box::new(event_handler));
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self).unwrap();
        self.event_handler = None;
    }

    /// Fires a [`GewyAppEvent`]. Handled by the handler supplied in the start() method.
    pub(crate) fn fire(&mut self, event: GewyAppEvent, event_loop: &ActiveEventLoop) {
        // Calls event handler
        let mut event_handler = self.event_handler.take().unwrap();
        let ctx = GewyContext { app: self };
        event_handler(event, ctx);
        self.event_handler = Some(event_handler);
        // Updates internal state reflecting recent changes
        self.init_window_views(event_loop, false);
    }

    // Initializes all window views if force is true.
    // Initializes all uninitialized window views if force is false.
    fn init_window_views(&mut self, event_loop: &ActiveEventLoop, force: bool) {
        for window in self.windows.values_mut() {
            if force || window.view.is_init() { continue };
            window.view = Late::Init(GewyWindowView::new(
                &self.instance,
                window.content_width,
                window.content_height,
                event_loop
            ));
        }
    }

    /// Adds a Window to the app.
    fn add_window(&mut self, mut window: GewyWindow) -> GewyWindowId {
        let root_id = window.node_tree.root_id();
        window.node_tree.render(root_id, &self.font_db);
        self.windows.insert(window)
    }

    /// Removes a window from the app.
    fn remove_window(&mut self, window_id: GewyWindowId) -> Option<GewyWindow> {
        self.windows.remove(window_id)
    }

    /// Gets mutable [`GewyWindowView`] using winit window id.
    fn get_window_with_winit_id(&mut self, winit_window_id: WindowId) -> Option<&mut GewyWindow> {
        self.windows.values_mut().find(|window| {
            let current_id = window.view.as_ref().unwrap().winit_window_id();
            current_id == winit_window_id
        })
    }

    /// Removes a [`GewyWindowView`] / [`GewyWindow`] pair using winit window id.
    fn remove_window_with_winit_id(&mut self, winit_window_id: WindowId, event_loop: &ActiveEventLoop) {
        self.windows.retain(|_, window| {
            let current_id = window.view.as_ref().unwrap().winit_window_id();
            current_id != winit_window_id
        });
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }
}

impl ApplicationHandler for GewyApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            self.fire(GewyAppEvent::Start, event_loop);
            self.started = true;
        }
        else {
            self.init_window_views(event_loop, true);
        }
        log::info!("{} window state created", self.windows.len());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        winit_window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::trace!("Handling window event: {event:?}");
        let Some(window) = self.get_window_with_winit_id(winit_window_id) else { return };
        let node_tree = &window.node_tree;
        let window_view = window.view.as_mut().unwrap();
        match event {
            WindowEvent::Resized(size) => {
                window_view.resize(size.width, size.height);
                window.node_tree.compute_layout_root(size.width as f32, size.height as f32);
            },
            WindowEvent::RedrawRequested => window_view.paint(node_tree),
            WindowEvent::CloseRequested => self.remove_window_with_winit_id(winit_window_id, event_loop),
            _ => {}
        }
    }
}



/// Allows for interaction with the [`GewyApp`] as it runs.
/// Can create and destroy windows.
pub struct GewyContext<'a> {
    app: &'a mut GewyApp,
}

impl<'a> GewyContext<'a> {
    pub fn window_count(&self) -> usize {
        self.app.windows.len()
    }
    pub fn add_window(&mut self, window: GewyWindow) -> GewyWindowId {
        self.app.add_window(window)
    }
    pub fn remove_window(&mut self, window_id: GewyWindowId) -> Option<GewyWindow> {
        self.app.remove_window(window_id)
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum GewyAppEvent {
    Start,
    Stop,
}