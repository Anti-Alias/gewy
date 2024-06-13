use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;
use crate::{GewyWindow, GewyWindowId, GewyWindowState};

/// Application that displays a gewy UI in a single winit window.
pub struct GewyApp {
    instance: Instance,
    window_states: Vec<GewyWindowState>,    // Parallel with 'windows'.
    windows: Vec<GewyWindow>,
    window_sequence: u64,
}

impl GewyApp {
    
    pub fn new() -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::default()),
            window_states: vec![],
            windows: vec![],
            window_sequence: 0,
        }
    }

    pub fn add_window(&mut self, window_state: GewyWindowState) -> GewyWindowId {
        self.window_states.push(window_state);
        let id = GewyWindowId(self.window_sequence);
        self.window_sequence += 1;
        id
    }

    /// Starts the application. Blocks until closed.
    pub fn start(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }

    fn get_window_mut(&mut self, window_id: WindowId) -> Option<(&mut GewyWindow, &mut GewyWindowState)> {
        for i in 0..self.windows.len() {
            if self.windows[i].window.id() == window_id {
                let window = &mut self.windows[i];
                let window_state = &mut self.window_states[i];
                return Some((window, window_state));
            }
        }
        None
    }

    fn remove_window(&mut self, window_id: WindowId, event_loop: &ActiveEventLoop) {
        for i in 0..self.windows.len() {
            if self.windows[i].window.id() == window_id {
                self.windows.remove(i);
                self.window_states.remove(i);
                if self.window_states.is_empty() {
                    event_loop.exit();
                }
                return;
            }
        }
    }
}

impl ApplicationHandler for GewyApp {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.windows = self.window_states
            .iter()
            .map(|window_state| GewyWindow::new(&self.instance, window_state, event_loop))
            .collect();
        log::info!("Window state created");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::trace!("Handling window event: {event:?}");
        let Some((window, window_state)) = self.get_window_mut(window_id) else { return };
        match event {
            WindowEvent::Resized(size) => window.resize(size.width, size.height),
            WindowEvent::RedrawRequested => window.draw(&window_state),
            WindowEvent::CloseRequested => self.remove_window(window_id, event_loop),
            _ => {}
        }
    }
}
