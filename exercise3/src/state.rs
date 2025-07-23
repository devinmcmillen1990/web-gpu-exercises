use std::sync::Arc;
use winit::window::Window;

pub struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let adapter_options = wgpu::RequestAdapterOptions::default();
        let adapter = instance.request_adapter(&adapter_options).await.unwrap();
        let device_descriptor = wgpu::DeviceDescriptor::default();
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we're going to create later.
            view_formats: vec![ self.surface_format.add_srgb_suffix() ],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    pub fn render(&mut self) {
        // Create texture view
        let surface_texture = self.surface.get_current_texture().expect("failed to acquire next swapchain texture");

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            // Without add_srgb_suffix() the image we will be working with might not be "gamma correct".
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        };

        let texture_view = surface_texture.texture.create_view(&texture_view_descriptor);

        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());
        
        // Create the renderpass which will clear the screen
        let color_attachment_operations = wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
            store: wgpu::StoreOp::Store,
        };


        let renderpass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: color_attachment_operations,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        let renderpass = encoder.begin_render_pass(&renderpass_descriptor);

        // If you wanted to call any drawing commands, they would go here.

        // End the render pass.
        drop(renderpass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}