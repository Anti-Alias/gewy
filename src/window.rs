use std::num::NonZeroUsize;
use std::pin::Pin;

use pollster::block_on;
use vello::peniko::Color;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{Adapter, CompositeAlphaMode, Device, DeviceDescriptor, Instance, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::{NodeTree, Widget};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, PartialOrd, Ord, Hash)]
pub struct GewyWindowId(pub(crate) u64);

pub struct GewyWindow {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub(crate) window: Pin<Box<Window>>,
    renderer: Renderer,
    scene: Scene,
}

impl GewyWindow {
    pub fn new(instance: &Instance, win_state: &GewyWindowState, event_loop: &ActiveEventLoop) -> Self {

        // WGPU setup
        let window = {
            let window_size: Size = PhysicalSize::new(win_state.width, win_state.height).into();
            let window_attrs = WindowAttributes::default().with_inner_size(window_size);
            let window = event_loop.create_window(window_attrs).unwrap();
            Pin::new(Box::new(window))
        };
        let surface: Surface<'static> = unsafe {
            let surface = instance.create_surface(&*window).unwrap();
            std::mem::transmute(surface)
        };
        let adapter_fut = instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        });
        let adapter = block_on(adapter_fut).unwrap();
        let surface_config = create_surface_config(win_state.width, win_state.height, &surface, &adapter);
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
        Self { device, queue, surface, surface_config, window, renderer, scene: Scene::new() }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn paint(&mut self, state: &GewyWindowState) {
        let Ok(surface_texture) = self.surface.get_current_texture() else { return };
        self.scene.reset();
        state.node_tree.paint_root(&mut self.scene);
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
}

/// Logical state of a [`GewyWindow`].
pub struct GewyWindowState {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) node_tree: NodeTree,
}

impl GewyWindowState {
    pub fn new(width: u32, height: u32, widget: impl Widget) -> Self {
        Self {
            width,
            height,
            node_tree: NodeTree::new(widget),
        }
    }
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
        present_mode: PresentMode::Mailbox,
        format,
        view_formats: vec![],
        usage: TextureUsages::RENDER_ATTACHMENT,
        width,
        height,
        desired_maximum_frame_latency: 2,
        alpha_mode: CompositeAlphaMode::Opaque,
    }
}