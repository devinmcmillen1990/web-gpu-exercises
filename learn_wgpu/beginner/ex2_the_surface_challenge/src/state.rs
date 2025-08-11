use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use wgpu::SurfaceError;

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    // Storing the mouse-adjustable color
    mouse_adjustable_color: wgpu::Color,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let wgpu_instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = wgpu_instance.create_surface(window.clone()).unwrap();
        let adapter = wgpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        }).await?;

        let size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,

            // Set that color's default to Black
            mouse_adjustable_color: wgpu::Color::BLACK,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match(code, is_pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            },
            _ => {}
        }
    }

    // Add function to modify the mouse-adjustable store color in the state.
    // NOTE: Only modifies the red and the green.
    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_adjustable_color.r = x / self.config.width as f64;
        self.mouse_adjustable_color.g = y / self.config.height as f64;
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Set the mouse-adjustable color instead of the blue-gray preset
                        load: wgpu::LoadOp::Clear(self.mouse_adjustable_color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}