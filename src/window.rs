use std::num::NonZeroUsize;
use std::pin::Pin;
use pollster::block_on;
use vello::peniko::Color;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{Adapter, CompositeAlphaMode, Device, DeviceDescriptor, Instance, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window as WinitWindow, WindowAttributes};
use crate::{FontDB, RawId, InputEvent, Store, Widget, UI};

/// A window within a [`GewyApp`](crate::GewyApp).
/// Contains a [`Widget`] tree, acting as a user interface.
pub struct Window {
    pub(crate) content_width: u32,
    pub(crate) content_height: u32,
    pub(crate) ui: UI,                 // UI DOM
    pub(crate) graphics: Option<WindowGraphics>,    // Graphical representation of window. Populated when window is created and when app is resumed (Android).
}

impl Window {
    pub fn new(content_width: u32, content_height: u32, widget: impl Widget) -> Self {
        Self {
            content_width,
            content_height,
            ui: UI::new(widget),
            graphics: None,
        }
    }

    /// Width of the content of the window. Excludes borders.
    pub fn content_width(&self) -> u32 { self.content_width }

    /// Height of the content of the window.  Excludes borders.
    pub fn content_height(&self) -> u32 { self.content_height }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.ui.compute_layout(width as f32, height as f32);
        self.content_width = width;
        self.content_height = height;
        if let Some(graphics) = &mut self.graphics {
            graphics.resize(width, height);
        }
    }

    pub(crate) fn render(&mut self, font_db: &FontDB, store: &Store) {
        self.ui.render(font_db, store);
    }

    pub(crate) fn compute_layout(&mut self) {
        self.ui.compute_layout(self.content_width as f32, self.content_height as f32);
    }

    pub(crate) fn paint(&mut self) {
        let graphics = self.graphics.as_mut().expect("Window graphics not present");
        graphics.paint(&self.ui);
    }

    pub fn fire_input_event(&mut self, event: InputEvent, store: &mut Store) {
        self.ui.fire_input_event(event, store);
    }

    pub fn inform_state_changes(&mut self, state_ids: &[RawId], font_db: &FontDB, store: &Store) {
        let num_updates = self.ui.inform_state_changes(state_ids);
        if num_updates != 0 {
            let root_id = self.ui.root_id();
            self.ui.render(font_db, store);
            self.ui.compute_layout_at(root_id, self.content_width as f32, self.content_height as f32);
            let Some(graphics) = &mut self.graphics else { return };
            graphics.winit_window.request_redraw();
        }
    }
}

/// Stores [`winit`] window and associated graphics state.
pub(crate) struct WindowGraphics {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    winit_window: Pin<Box<WinitWindow>>,
    renderer: Renderer,
    scene: Scene,
}

impl WindowGraphics {
    pub fn new(
        instance: &Instance,
        content_width: u32,
        content_height: u32,
        event_loop: &ActiveEventLoop,
    ) -> Self {

        // WGPU setup
        let winit_window = {
            let window_size: Size = PhysicalSize::new(content_width, content_height).into();
            let window_attrs = WindowAttributes::default().with_inner_size(window_size);
            let window = event_loop.create_window(window_attrs).unwrap();
            Pin::new(Box::new(window))
        };
        let surface: Surface<'static> = unsafe {
            let surface = instance.create_surface(&*winit_window).unwrap();
            std::mem::transmute(surface)
        };
        let adapter_fut = instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        });
        let adapter = block_on(adapter_fut).unwrap();
        let surface_config = create_surface_config(content_width, content_height, &surface, &adapter);
        let device_queue_fut = adapter.request_device(&DeviceDescriptor::default(), None);
        let (device, queue) = block_on(device_queue_fut).unwrap();
        surface.configure(&device, &surface_config);

        // Vello setup
        let renderer = Renderer::new(&device, RendererOptions {
            surface_format: Some(surface_config.format),
            use_cpu: false,
            antialiasing_support: AaSupport::all(),
            num_init_threads: NonZeroUsize::new(1),
        }).unwrap();
        Self { device, queue, surface, surface_config, winit_window, renderer, scene: Scene::new() }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn paint(&mut self, node_tree: &UI) {
        self.scene.reset();
        node_tree.paint_root(&mut self.scene);
        let Ok(surface_texture) = self.surface.get_current_texture() else { return };
        self.renderer.render_to_surface(
            &self.device,
            &self.queue,
            &self.scene,
            &surface_texture,
            &RenderParams {
                base_color: Color::BLACK,
                width: self.surface_config.width,
                height: self.surface_config.height,
                antialiasing_method: AaConfig::Msaa16,
            }
        ).unwrap();
        surface_texture.present();
    }

    pub(crate) fn winit_id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }
}

slotmap::new_key_type! {
    pub struct WindowId;
}

fn create_surface_config(
    width: u32,
    height: u32,
    surface: &Surface,
    adapter: &Adapter
) -> SurfaceConfiguration {
    let caps = surface.get_capabilities(adapter);
    let format = caps.formats
        .iter()
        .find(|format| format.is_srgb())
        .expect("No suitable texture format found")
        .clone();
    SurfaceConfiguration {
        present_mode: PresentMode::AutoNoVsync,
        format,
        view_formats: vec![],
        usage: TextureUsages::RENDER_ATTACHMENT,
        width,
        height,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::Opaque,
    }
}