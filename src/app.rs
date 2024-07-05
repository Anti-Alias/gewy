use slotmap::SlotMap;
use taffy::{AlignItems, Dimension, JustifyContent};
use wgpu::{Instance, InstanceDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::WindowId as WinitWindowId;
use crate::{Div, FontDB, InputMessage, Store, View, Window, WindowGraphics, WindowId, UI};

/// Application that displays a gewy UI in a single winit window.
pub struct App {
    fonts: FontDB,
    store: Store,
    listeners: Vec<Box<dyn AppListener>>,
}
impl App {
    
    pub fn new(fonts: FontDB) -> Self {
        Self {
            fonts,
            store: Store::new(),
            listeners: vec![],
        }
    }

    pub fn on<F, R>(mut self, event: AppEvent, callback: F) -> Self
    where
        F: Fn(&mut AppCtx) -> R + 'static
    {
        let listener = move |ctx: &mut AppCtx, evt: AppEvent| {
            if evt == event {
                callback(ctx);
            }
        };
        self.listeners.push(Box::new(listener));
        self
    }

    pub fn add_listener(&mut self, listener: impl AppListener) {
        self.listeners.push(Box::new(listener));
    }

    /// Starts the application. Blocks until closed.
    pub fn start(self) {
        let ctx = AppCtx {
            store: self.store,
            instance: Instance::new(InstanceDescriptor::default()),
            windows: SlotMap::default(),
            fonts: self.fonts,
        };
        let event_loop: EventLoop<AppEvent> = EventLoop::with_user_event().build().unwrap();
        let mut listener = AppHandler::new(ctx, self.listeners, event_loop.create_proxy());
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut listener).unwrap();
    }
}

struct AppHandler {
    ctx: AppCtx,
    listeners: Vec<Box<dyn AppListener>>,
    _event_handler: Option<Box<dyn FnMut(&mut AppCtx, AppEvent)>>,
    proxy: EventLoopProxy<AppEvent>,
    started: bool,
}

impl AppHandler {
    fn new(
        ctx: AppCtx,
        listeners: Vec<Box<dyn AppListener>>,
        proxy: EventLoopProxy<AppEvent>,
    ) -> Self {
        Self {
            ctx,
            listeners,
            _event_handler: None,
            proxy,
            started: false,
        }
    }
}

impl ApplicationHandler<AppEvent> for AppHandler {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            log::info!("App starting");
            self.proxy.send_event(AppEvent::Start).unwrap();
        }
        else {
            log::info!("App resuming");
            self.ctx.create_window_graphics(event_loop);
        }
        log::info!("{} window(s) created", self.ctx.windows.len());
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.ctx.destroy_window_graphics();
        log::info!("App suspended");
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
            WindowEvent::CursorEntered { .. }               => window.fire_input_event(InputMessage::CursorEntered, store),
            WindowEvent::CursorLeft { .. }                  => window.fire_input_event(InputMessage::CursorLeft, store),
            WindowEvent::CursorMoved { position, .. }       => window.fire_input_event(InputMessage::CursorMoved { x: position.x as f32, y: position.y as f32 }, store),
            WindowEvent::MouseInput { state, button, .. }   => window.fire_input_event(InputMessage::from_winit_mouse(state, button), store),
            WindowEvent::Resized(size)                      => window.resize(size.width, size.height),
            WindowEvent::RedrawRequested                    => window.paint(),
            WindowEvent::CloseRequested                     => ctx.remove_window_winit(winit_id, event_loop),
            _ => {}
        }

        // Broadcasts state changes
        let states_changed = self.ctx.store.handle_events();
        if !states_changed.is_empty() {
            for window in self.ctx.windows.values_mut() {
                window.inform_state_changes(&states_changed, &self.ctx.fonts, &self.ctx.store);
            }
        }

        // Initializes graphics of windows if any new one was created
        self.ctx.create_window_graphics(event_loop);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::Start => {
                for listener in &self.listeners {
                    listener.handle(&mut self.ctx, AppEvent::Start);
                }
                self.ctx.create_window_graphics(event_loop);
                self.ctx.render_windows();
                log::info!("App started");
                self.started = true;
            },
            AppEvent::Stop => {},
        }
    }
}



/// Object that allows for manipulating the app as it runs.
pub struct AppCtx {
    pub store: Store,
    instance: Instance,
    windows: SlotMap<WindowId, Window>,
    fonts: FontDB,
}

impl AppCtx {

    pub fn create_window<F>(&mut self, width: u32, height: u32, view_fn: F) -> WindowId
    where
        F: FnOnce(&mut Store, &mut View)
    {
        // Builds root div
        let mut div = Div::default();
        div.style.size.width = Dimension::Percent(1.0);
        div.style.size.height = Dimension::Percent(1.0);
        div.style.justify_content = Some(JustifyContent::Center);
        div.style.align_items = Some(AlignItems::Center);

        // Builds initial UI
        let mut ui = UI::new(div);
        let ui_root = ui.root_id();
        let mut view = View::new(&mut ui, ui_root, &self.fonts);
        view_fn(&mut self.store, &mut view);
        ui.init(ui_root, &self.fonts);

        // Shows window
        self.windows.insert(Window::new(width, height, ui))
    }

    pub fn remove_window(&mut self, window_id: WindowId) -> Option<Window> {
        self.windows.remove(window_id)
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
                window.width,
                window.height,
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
            window.render(&self.fonts, &self.store);
            window.compute_layout();
        }
    }
}

pub trait AppListener: 'static {
    fn handle(&self, ctx: &mut AppCtx, event: AppEvent);
}

impl<F> AppListener for F
where
    F: Fn(&mut AppCtx, AppEvent) + 'static
{
    fn handle(&self, ctx: &mut AppCtx, event: AppEvent) {
        self(ctx, event);
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AppEvent {
    Start,
    Stop,
}