use crate::{AppEvent, WindowId};
use pollster::FutureExt;
use std::sync::Arc;
use vello::kurbo::Affine;
use vello::util::{RenderContext, RenderSurface};
use vello::{Renderer, RendererOptions, Scene};
use wgpu::{PresentMode, TextureViewDescriptor};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window as WinitWindow, WindowAttributes};

const CLEAR_COLOR: vello::peniko::Color = vello::peniko::Color::from_rgba8(100, 100, 100, 255);

pub struct Window {
    // Vello
    renderer: Renderer,
    surface: RenderSurface<'static>,
    scene: Scene,
    // Winit
    window: Arc<WinitWindow>,
    window_attributes: WindowAttributes,
    proxy: EventLoopProxy<AppEvent>,
}

impl Window {

    pub(crate) fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        context: &mut RenderContext,
        proxy: EventLoopProxy<AppEvent>,
    ) -> Self {
        let window = event_loop.create_window(window_attributes.clone()).unwrap();
        let window = Arc::new(window);
        let surface = context
            .create_surface(
                window.clone(),
                window.inner_size().width,
                window.inner_size().height,
                PresentMode::Fifo,
            )
            .block_on()
            .unwrap();
        let device = &context.devices[surface.dev_id].device;
        let renderer = Renderer::new(device, RendererOptions::default()).unwrap();
        Self {
            window,
            window_attributes,
            renderer,
            proxy,
            surface,
            scene: Scene::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, context: &RenderContext) {
        let width = width.max(1);
        let height = height.max(1);
        context.resize_surface(&mut self.surface, width, height);
    }

    pub(crate) fn recreate(&mut self, event_loop: &ActiveEventLoop, context: &mut RenderContext) {
        let window_attributes = self.window_attributes.clone();
        *self = Self::new(event_loop, window_attributes, context, self.proxy.clone());
    }

    pub(crate) fn redraw(&mut self, context: &RenderContext) {
        
        // Paints vello scene
        self.scene.reset();
        self.scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            vello::peniko::Color::from_rgb8(242, 140, 168),
            None,
            &vello::kurbo::Circle::new((420.0, 200.0), 120.0),
        );

        // Renders to offscreen texture provided by surface 
        let device_handle = &context.devices[self.surface.dev_id];
        self.renderer.render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            &self.scene,
            &self.surface.target_view,
            &vello::RenderParams {
                base_color: CLEAR_COLOR,
                width: self.surface.config.width,
                height: self.surface.config.height,
                antialiasing_method: vello::AaConfig::Msaa16,
            },
        )
        .unwrap();

        // Takes main surface texture off the "swap chain"
        let surface_tex = self.surface.surface.get_current_texture().unwrap();
        let surface_tex_view = surface_tex.texture.create_view(&TextureViewDescriptor::default());

        // Blits offscreen texture onto the surface texture
        let encoder_desc = &wgpu::CommandEncoderDescriptor { label: Some("Surface Blit") };
        let mut encoder = device_handle.device.create_command_encoder(encoder_desc);
        self.surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &self.surface.target_view,
            &surface_tex_view,
        );
        device_handle.queue.submit([encoder.finish()]);

        // Returns surface back to "swap chain"
        surface_tex.present();
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }
}

fn select_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap()
}
