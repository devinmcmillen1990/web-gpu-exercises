use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance_descriptor = wgpu::InstanceDescriptor::default();

        // The instance is a handle to our GPU
        // Instance is responsible for creating the adapter and the surface
        let instance = wgpu::Instance::new(&instance_descriptor);

        let surface = instance.create_surface(window.clone()).unwrap();

        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),     // Favors Low-Power
            compatible_surface: Some(&surface),                     // Tells WGPU to find an adapter that can present the supplied surface
            force_fallback_adapter: false,
        };

        // The adapter is used to create the device and the queue.
        let adapter = instance.request_adapter(&request_adapter_options).await?;

        let device_descriptor = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),     // allows us to specify what extra features we want. https://docs.rs/wgpu/latest/wgpu/struct.Features.html
            required_limits: wgpu::Limits::default(),       // describes the limit of certain types of resources that we can create
            memory_hints: Default::default(),               // provides the adapter with a preferred memory allocation strategy, if supported. https://wgpu.rs/doc/wgpu/enum.MemoryHints.html
            trace: wgpu::Trace::Off,
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,          // textures will be used to write to the screen
            format: surface_format,                                 // defines how SurfaceTextures will be stored on the GPU
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],    // uses wgpu::PresentMode enum, which determines how to sync the surface with the display
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],                                   // a list of TextureFormats that you can use when creating TextureViews
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
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

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>  {
        self.window.request_redraw();

        // We can't render if the surface isn't configured.
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;    // wait for the surface to provide a new SurfaceTexture that we will render to

        let texture_view_descriptor = wgpu::TextureViewDescriptor::default();
        let view = output.texture.create_view(&texture_view_descriptor);        // creates TextureView with default settings. We need to do this because we want to control how the render code interacts with the texture.

        /*
            We also need to create a CommandEncoder to create the actual commands to send to the GPU. 
            Most modern graphics frameworks expect commands to be stored in a command buffer before being sent to the GPU. 
            The encoder builds a command buffer that we can then send to the GPU.
        */

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let mut encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        // The block tells Rust to drop any variables within it when the code leaves that scope, thus releasing the 
        // mutable borrow on encoder and allowing us to finish() it. If you don't like the {}, you can also use drop
        // (render_pass) to achieve the same effect.
        {
            let renderpass_descriptor = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {                             // describe where we are going to draw our color to.
                    view: &view,                                                                        // informs wgpu what texture to save the colors to
                    resolve_target: None,                                                               // the texture that will receive the resolved output. This will be the same as view unless multisampling is enabled.
                    ops: wgpu::Operations {                                                             // tells wgpu what to do with the colors on the screen (specified by view)
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0, }),     // tells wgpu how to handle colors stored from the previous frame
                        store: wgpu::StoreOp::Store,                                                    // tells wgpu whether we want to store the rendered results to the Texture behind our TextureView (in this case, it's the SurfaceTexture). We use StoreOp::Store as we do want to store our render results
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            };

            let _renderpass = encoder.begin_render_pass(&renderpass_descriptor);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}