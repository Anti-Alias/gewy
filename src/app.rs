use crate::{Window, WindowAttributes, WindowId};
use vello::util::RenderContext;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, PhysicalKey};

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
    let mut app_handler = AppHandler::new(app, proxy);
    event_loop.run_app(&mut app_handler).unwrap();
}

/// Internal implementation of [`AppHandler`].
/// Wraps an [`App`], and forwards winit events to it. 
struct AppHandler<A: App> {
    app: A,
    context: RenderContext,
    started: bool,
    windows: Vec<Window>,
    proxy: EventLoopProxy<AppEvent>,
}

impl<A: App> AppHandler<A> {

    // Called when the app first starts
    fn start(&mut self, event_loop: &ActiveEventLoop) {
        let ctx = AppCtx { event_loop, proxy: &self.proxy, context: &mut self.context, windows: &mut self.windows };
        self.app.start(ctx);
        if self.windows.is_empty() {
            event_loop.exit();
        }
        else {
            self.started = true;
        }
    }

    /// Called when app resumes from a suspended state 
    fn resume(&mut self, event_loop: &ActiveEventLoop) {
        for window in self.windows.iter_mut() {
            window.recreate(event_loop, &mut self.context);
        }
    }

    // Handles key events on a focused window 
    fn key_event(&mut self, event: KeyEvent, event_loop: &ActiveEventLoop) {
        if event.state != ElementState::Pressed { return };
        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyQ) => event_loop.exit(),
            _ => {},
        }
    }
}

impl<A: App> ApplicationHandler<AppEvent> for AppHandler<A> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            self.start(event_loop);
        } else {
            self.resume(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {

            WindowEvent::KeyboardInput { event, .. } => self.key_event(event, event_loop),

            WindowEvent::CloseRequested => {
                self.windows.retain(|window| window.id() != window_id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }

            WindowEvent::RedrawRequested => {
                log::trace!("Redrawing");
                let window = self
                    .windows
                    .iter_mut()
                    .find(|window| window.id() == window_id);
                let Some(window) = window else { return };
                window.redraw(&self.context);
            }

            WindowEvent::Resized(size) => {
                log::trace!("Resized");
                let window = self
                    .windows
                    .iter_mut()
                    .find(|window| window.id() == window_id);
                let Some(window) = window else { return };
                window.resize(size.width, size.height, &self.context);
            }

            _ => {},
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::Exit => event_loop.exit(),
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let ctx = AppCtx { event_loop, proxy: &self.proxy, context: &mut self.context, windows: &mut self.windows };
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
            context: RenderContext::new(),
        }
    }
}

/// Object that allows for talking to the application runtime.
pub struct AppCtx<'a> {
    event_loop: &'a ActiveEventLoop,
    proxy:      &'a EventLoopProxy<AppEvent>,
    context:    &'a mut RenderContext,
    windows:    &'a mut Vec<Window>,
}

impl<'a> AppCtx<'a> {
    /// Creates a new window.
    pub fn create_window(&mut self, attributes: WindowAttributes) -> WindowId {
        let window = Window::new(
            self.event_loop,
            attributes,
            self.context,
            self.proxy.clone(),
        );
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
        self.windows.iter().find(|window| window.id() == id)
    }


    /// Requests that the application close.
    pub fn exit(&mut self) {
        self.event_loop.exit();
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AppEvent { Exit }
