use slotmap::SlotMap;
use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;
use crate::{FontDB, GewyWindow, GewyWindowId, GewyWindowView, Late};

/// Application that displays a gewy UI in a single winit window.
pub struct GewyApp {
    instance: Instance,
    windows: SlotMap<GewyWindowId, GewyWindow>,    // Parallel with 'windows'.
    font_db: FontDB,
    event_handler: Option<Box<dyn FnMut(AppEvent)>>,
}

impl GewyApp {
    
    pub fn new(font_db: FontDB) -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::default()),
            windows: SlotMap::default(),
            font_db,
            event_handler: None,
        }
    }

    /// Adds a Window to the app.
    /// Window will appear once the app starts.
    pub fn add_window(&mut self, mut window: GewyWindow) -> GewyWindowId {
        let root_id = window.node_tree.root_id();
        window.node_tree.render(root_id, &self.font_db);
        self.windows.insert(window)
    }

    /// Removes a window from the app.
    /// Window will not appear when the app starts.
    pub fn remove_window(&mut self, window_id: GewyWindowId) -> Option<GewyWindow> {
        self.windows.remove(window_id)
    }

    /// Starts the application. Blocks until closed.
    pub fn start(mut self) {
        if self.windows.is_empty() { return }
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }

    /// Gets mutable [`GewyWindowView`] using winit window id.
    fn get_window_winit_mut(&mut self, winit_window_id: WindowId) -> Option<&mut GewyWindow> {
        self.windows.values_mut().find(|window| {
            let current_id = window.view.as_ref().unwrap().winit_window_id();
            current_id == winit_window_id
        })
    }

    /// Removes a [`GewyWindowView`] / [`GewyWindow`] pair using winit window id.
    fn remove_window_view(&mut self, winit_window_id: WindowId, event_loop: &ActiveEventLoop) {
        self.windows.retain(|_, window| {
            let current_id = window.view.as_ref().unwrap().winit_window_id();
            current_id == winit_window_id
        });
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }
}

impl ApplicationHandler for GewyApp {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        for window in self.windows.values_mut() {
            window.view = Late::Init(GewyWindowView::new(
                &self.instance,
                window.content_width,
                window.content_height, event_loop
            ));
        }
        log::info!("{} window state created", self.windows.len());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::trace!("Handling window event: {event:?}");
        let Some(window) = self.get_window_winit_mut(window_id) else { return };
        let window_view = window.view.as_mut().unwrap();
        match event {
            WindowEvent::Resized(size) => {
                window_view.resize(size.width, size.height);
                window.node_tree.compute_layout_root(size.width as f32, size.height as f32);
            },
            WindowEvent::RedrawRequested => window_view.paint(&window),
            WindowEvent::CloseRequested => self.remove_window(window_id, event_loop),
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
    fn add_window(&mut self, mut window_state: GewyWindow) -> GewyWindowId {
        self.app.add_window(window_state)
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AppEvent {
    Start,
}