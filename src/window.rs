use std::sync::Arc;

use winit::{event_loop::ActiveEventLoop, window::{Window as WinitWindow, WindowAttributes}};
use pollster::FutureExt;
use wgpu::*;
use crate::WindowId;

pub struct Window {
    window: Arc<WinitWindow>,
    window_attributes: WindowAttributes,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}

impl Window {

    pub(crate) fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        instance: &Instance,
    ) -> Self {
        let window = event_loop.create_window(window_attributes.clone()).unwrap();
        let window = Arc::new(window);
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = select_adapter(instance, &surface);
        let surface_config = create_surface_config(&surface, &adapter, window.inner_size().width, window.inner_size().height);
        let (device, queue) = adapter 
            .request_device(&DeviceDescriptor::default())
            .block_on()
            .unwrap();
        surface.configure(&device, &surface_config);
        Self { window, window_attributes, surface, surface_config, device, queue }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub(crate) fn recreate(&mut self, event_loop: &ActiveEventLoop, instance: &Instance) {
        let window_attributes = self.window_attributes.clone();
        *self = Self::new(event_loop, window_attributes, instance);
    }

    pub(crate) fn redraw(&mut self) {
        let encoder_desc = CommandEncoderDescriptor { label: Some("Window Encoder") };
        let Ok(surface_texture) = self.surface.get_current_texture() else { return };
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&encoder_desc);
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Window Render Pass"),
                color_attachments: &[
                    Some(RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(CLEAR_COLOR),
                            store: StoreOp::Discard,
                        },
                    })
                ],
                ..Default::default()
            });
        }
        let command_buffers = std::iter::once(encoder.finish());
        self.queue.submit(command_buffers);
        surface_texture.present();
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }
}

fn select_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap()
}

fn create_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    width: u32,
    height: u32,
) -> SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    SurfaceConfiguration {
        width,
        height,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

const CLEAR_COLOR: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
