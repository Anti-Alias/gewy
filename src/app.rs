use wgpu::{Backends, Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use crate::{Window, WindowAttributes, WindowId};

/// Trait representing an application.
pub trait App {

    /// Called when application first starts.
    /// Generally, initial window(s) will be created here.
    fn start(&mut self, _ctx: AppCtx) {}

    /// Cleanup logic for when application exits. 
    fn exit(&mut self, _ctx: AppCtx) {}
}


/// Runs the application, blocking until the application quits.
pub fn run_app(app: impl App) {
    let event_loop = EventLoop::<AppEvent>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();
    let mut handler = AppHandler::new(app, proxy);
    event_loop.run_app(&mut handler).unwrap();
}

/// Wraps an [`App`] instance, and implements [`ApplicationHandler`].
struct AppHandler<A: App> {
    app: A,
    instance: Instance,
    started: bool,
    windows: Vec<Window>,
    proxy: EventLoopProxy<AppEvent>,
}
impl<A: App> ApplicationHandler<AppEvent> for AppHandler<A> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut ctx = AppCtx {
            event_loop,
            instance: &self.instance,
            windows: &mut self.windows,
            proxy: &self.proxy,
        };
        if !self.started {
            self.app.start(ctx);
            self.started = true;
        }
        else {
            ctx.recreate_windows();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.windows.retain(|window| window.id() != window_id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            },
            WindowEvent::RedrawRequested => {
                log::trace!("Redrawing");
                let window = self.windows.iter_mut().find(|window| window.id() == window_id);
                let Some(window) = window else { return };
                window.redraw();
            },
            WindowEvent::Resized(size) => {
                log::trace!("Resized");
                let window = self.windows.iter_mut().find(|window| window.id() == window_id);
                let Some(window) = window else { return };
                window.resize(size.width, size.height);
            },
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::Exit => event_loop.exit(),
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let ctx = AppCtx {
            event_loop,
            instance: &self.instance,
            windows: &mut self.windows,
            proxy: &self.proxy,
        };
        self.app.exit(ctx);    
    }

}

impl<A: App> AppHandler<A> {
    fn new(app: A, proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            app,
            proxy,
            started: false,
            windows: vec![],
            instance: Instance::new(&InstanceDescriptor {
                backends: Backends::VULKAN | Backends::DX12 | Backends::METAL,
                ..Default::default()
            }),
        }
    }
}

/// Object that allows for talking to the application runtime.
pub struct AppCtx<'a> {
    event_loop: &'a ActiveEventLoop,
    proxy: &'a EventLoopProxy<AppEvent>,
    instance: &'a Instance,
    windows: &'a mut Vec<Window>,
}

impl<'a> AppCtx<'a> {

    /// Creates a new window.
    pub fn create_window(&mut self, attributes: WindowAttributes) -> WindowId {
        let window = Window::new(self.event_loop, attributes, self.instance, self.proxy.clone());
        let window_id = window.id();
        self.windows.push(window);
        window_id
    }

    /// Gets a [`Window`] by id.
    pub fn windows(&self) -> &[Window] {
        &self.windows
    }

    /// Gets a [`Window`] by id.
    pub fn window(&self, id: WindowId) -> Option<&Window> {
        self.windows
            .iter()
            .find(|window| window.id() == id)
    }

    /// Restores windows when application resumes. 
    fn recreate_windows(&mut self) {
        for window in self.windows.iter_mut() {
            window.recreate(&self.event_loop, &self.instance);
        }
    }

    /// Requests that the application close.
    pub fn exit(&mut self) {
        self.event_loop.exit();
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AppEvent { Exit }
