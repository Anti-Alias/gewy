use slotmap::SlotMap;
use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::WindowId as WinitWindowId;
use crate::{FontDB, InputEvent, Store, Window, WindowGraphics, WindowId};

/// Application that displays a gewy UI in a single winit window.
pub struct App {
    font_db: FontDB,
    store: Store,
}

impl App {
    
    pub fn new(font_db: FontDB) -> Self {
        Self {
            font_db,
            store: Store::new(),
        }
    }

    /// Starts the application. Blocks until closed.
    pub fn start<F>(self, mut startup: F)
    where
        F: FnMut(&mut AppCtx),
    {
        let mut ctx = AppCtx {
            store: self.store,
            instance: Instance::new(InstanceDescriptor::default()),
            windows: SlotMap::default(),
            font_db: self.font_db,
            event_handler: None,
        };
        startup(&mut ctx);
        

        let event_loop: EventLoop<AppEvent> = EventLoop::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        let mut listener = AppListener::new(ctx, proxy);
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut listener).unwrap();
    }
}

struct AppListener {
    ctx: AppCtx,
    _event_handler: Option<Box<dyn FnMut(&mut AppCtx, AppEvent)>>,
    _proxy: EventLoopProxy<AppEvent>,
    started: bool,
}

impl AppListener {
    fn new(ctx: AppCtx, proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            ctx,
            _event_handler: None,
            _proxy: proxy,
            started: false,
        }
    }
}

impl ApplicationHandler<AppEvent> for AppListener {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            self.ctx.create_window_graphics(event_loop);
            self.ctx.render_windows();
            self.started = true;
        }
        else {
            self.ctx.create_window_graphics(event_loop);
        }
        log::info!("{} window(s) created", self.ctx.windows.len());
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.ctx.destroy_window_graphics();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        winit_id: WinitWindowId,
        event: WindowEvent,
    ) {

        // Updates relevant window
        log::trace!("Handling window event: {event:?}");
        let ctx = &mut self.ctx;
        let Some((window, store)) = ctx.get_window_winit(winit_id) else { return };
        match event {
            WindowEvent::CursorEntered { .. }               => window.fire_input_event(InputEvent::CursorEntered, store),
            WindowEvent::CursorLeft { .. }                  => window.fire_input_event(InputEvent::CursorLeft, store),
            WindowEvent::CursorMoved { position, .. }       => window.fire_input_event(InputEvent::CursorMoved { x: position.x as f32, y: position.y as f32 }, store),
            WindowEvent::MouseInput { state, button, .. }   => window.fire_input_event(InputEvent::from_winit_mouse(state, button), store),
            WindowEvent::Resized(size)                      => window.resize(size.width, size.height),
            WindowEvent::RedrawRequested                    => window.paint(),
            WindowEvent::CloseRequested                     => ctx.remove_window_winit(winit_id, event_loop),
            _ => {}
        }

        // Broadcasts state changes
        let states_changed = self.ctx.store.handle_events();
        if !states_changed.is_empty() {
            for window in self.ctx.windows.values_mut() {
                window.inform_state_changes(&states_changed, &self.ctx.font_db, &self.ctx.store);
            }
        }

        // Initializes graphics of windows if any new one was created
        self.ctx.create_window_graphics(event_loop);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        let Some(mut event_handler) = self.ctx.event_handler.take() else { return };
        event_handler(&mut self.ctx, event);
        self.ctx.event_handler = Some(event_handler);
        self.ctx.create_window_graphics(event_loop);
    }
}



/// Object that allows for manipulating the app as it runs.
pub struct AppCtx {
    pub store: Store,
    instance: Instance,
    windows: SlotMap<WindowId, Window>,
    font_db: FontDB,
    event_handler: Option<Box<dyn FnMut(&mut Self, AppEvent)>>,
}

impl AppCtx {

    pub fn add_window(&mut self, window: Window) -> WindowId {
        self.windows.insert(window)
    }

    pub fn remove_window(&mut self, window_id: WindowId) -> Option<Window> {
        self.windows.remove(window_id)
    }

    pub fn set_event_handler(&mut self, event_handler: impl FnMut(&mut AppCtx, AppEvent) + 'static) {
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

    fn destroy_window_graphics(&mut self) {
        for window in self.windows.values_mut() {
            window.graphics = None;
        }
    }

    fn render_windows(&mut self) {
        for window in self.windows.values_mut() {
            window.render(&self.font_db, &self.store);
            window.compute_layout();
        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AppEvent {
    Start,
    Stop,
}