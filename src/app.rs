use std::any::Any;

use slotmap::SlotMap;
use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::WindowId as WinitWindowId;
use crate::{FontDB, FromStore, Id, InputEvent, Store, Window, WindowGraphics, WindowId};

/// Application that displays a gewy UI in a single winit window.
pub struct App {
    instance: Instance,
    windows: SlotMap<WindowId, Window>,
    font_db: FontDB,
    store: Store,
    event_handler: Option<Box<dyn FnMut(&mut Self, AppEvent)>>,
}

impl App {
    
    pub fn new(font_db: FontDB) -> Self {
        Self {
            instance: Instance::new(InstanceDescriptor::default()),
            windows: SlotMap::default(),
            font_db,
            event_handler: None,
            store: Store::new(),
        }
    }

    /// Starts the application. Blocks until closed.
    pub fn start<F>(mut self, mut startup: F)
    where
        F: FnMut(&mut AppCtx),
    {
        let mut ctx = AppCtx { app: &mut self };
        startup(&mut ctx);

        let event_loop: EventLoop<AppEvent> = EventLoop::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        let mut listener = AppListener::new(self, proxy);
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut listener).unwrap();
    }

    fn create_window_graphics(&mut self, event_loop: &ActiveEventLoop) {
        for window in self.windows.values_mut() {
            if window.graphics.is_some() { continue };
            window.graphics = Some(WindowGraphics::new(
                &self.instance,
                window.content_width,
                window.content_height,
                event_loop
            ));
        }
    }

    fn render_windows(&mut self) {
        for window in self.windows.values_mut() {
            window.render(&self.font_db, &self.store);
            window.compute_layout();
        }
    }

    fn destroy_window_graphics(&mut self) {
        for window in self.windows.values_mut() {
            window.graphics = None;
        }
    }

    /// Adds a Window to the app.
    fn add_window(&mut self, window: Window) -> WindowId {
        self.windows.insert(window)
    }

    /// Removes a window from the app.
    fn remove_window(&mut self, window_id: WindowId) -> Option<Window> {
        self.windows.remove(window_id)
    }

    pub fn set_event_handler(&mut self, event_handler: impl FnMut(&mut Self, AppEvent) + 'static) {
        self.event_handler = Some(Box::new(event_handler));
    }

    fn get_window_winit(&mut self, winit_id: WinitWindowId) -> Option<(&mut Window, &mut Store)> {
        self.windows.values_mut()
            .find(|window| {
                let current_id = window.graphics.as_ref().unwrap().winit_id();
                current_id == winit_id
            })
            .map(|window| (window, &mut self.store))
    }

    fn remove_window_winit(&mut self, winit_id: WinitWindowId, event_loop: &ActiveEventLoop) {
        self.windows.retain(|_, window| {
            let current_id = window.graphics.as_ref().unwrap().winit_id();
            current_id != winit_id
        });
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }
}

struct AppListener {
    app: App,
    proxy: EventLoopProxy<AppEvent>,
    started: bool,
}

impl AppListener {
    fn new(app: App, proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            app,
            proxy,
            started: false,
        }
    }
}

impl ApplicationHandler<AppEvent> for AppListener {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            self.app.create_window_graphics(event_loop);
            self.app.render_windows();
            self.started = true;
        }
        else {
            self.app.create_window_graphics(event_loop);
        }
        log::info!("{} window(s) created", self.app.windows.len());
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.destroy_window_graphics();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        winit_id: WinitWindowId,
        event: WindowEvent,
    ) {

        // Updates relevant window
        log::trace!("Handling window event: {event:?}");
        let Some((window, store)) = self.app.get_window_winit(winit_id) else { return };
        match event {
            WindowEvent::CursorEntered { .. }               => window.fire_input_event(InputEvent::CursorEntered, store),
            WindowEvent::CursorLeft { .. }                  => window.fire_input_event(InputEvent::CursorLeft, store),
            WindowEvent::CursorMoved { position, .. }       => window.fire_input_event(InputEvent::CursorMoved { x: position.x as f32, y: position.y as f32 }, store),
            WindowEvent::MouseInput { state, button, .. }   => window.fire_input_event(InputEvent::from_winit_mouse(state, button), store),
            WindowEvent::Resized(size)                      => window.resize(size.width, size.height),
            WindowEvent::RedrawRequested                    => window.paint(),
            WindowEvent::CloseRequested                     => self.app.remove_window_winit(winit_id, event_loop),
            _ => {}
        }

        // Broadcasts state changes
        let states_changed = self.app.store.update();
        if !states_changed.is_empty() {
            for window in self.app.windows.values_mut() {
                window.inform_state_changes(&states_changed, &self.app.font_db, &self.app.store);
                //window.ui.print_diagnostics();
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        let Some(mut event_handler) = self.app.event_handler.take() else { return };
        event_handler(&mut self.app, event);
        self.app.event_handler = Some(event_handler);
        self.app.create_window_graphics(event_loop);
    }
}



/// Object that allows for manipulating an [`App`] as it runs.
pub struct AppCtx<'a> {
    app: &'a mut App,
}

impl<'a> AppCtx<'a> {

    /// Creates a window, returning its id.
    pub fn add_window(&mut self, window: Window) -> WindowId {
        self.app.add_window(window)
    }

    /// Removes a window.
    pub fn remove_window(&mut self, window_id: WindowId) -> Option<Window> {
        self.app.remove_window(window_id)
    }

    /// Creates a state object, returning its id.
    pub fn create_state<S: Any>(&mut self, value: S) -> Id<S> {
        self.app.store.create(value)
    }

    /// Creates a state object, returning its id.
    pub fn init_state<S: Any + FromStore>(&mut self) -> Id<S> {
        self.app.store.init()
    }

    /// Gets read-only access to the value of a state object.
    pub fn state<S, I>(&self, state_id: I) -> &S
    where
        S: Any,
        I: AsRef<Id<S>>,
    {
        self.app.store.get(state_id.as_ref())
    }

    /// Gets write access to the value of a state object.
    pub fn state_mut<S, I>(&mut self, id: I) -> &mut S
    where
        S: Any,
        I: AsRef<Id<S>>,
    {
        self.app.store.get_mut(id.as_ref())
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AppEvent {
    Start,
    Stop,
}