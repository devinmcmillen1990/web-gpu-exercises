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
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let surface = instance.create_surface(window.clone()).unwrap();
        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&request_adapter_options).await?;
        let device_descriptor = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };

        let render_pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),                                   // @vertex decorated function in shader.wgsl
                buffers: &[],                                                   // tells wgpu what type of vertices we want to pass to the vertex shader. We're specifying the vertices in the vertex shader itself, so we'll leave this empty.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),                                   // @fragment decorated function in shader.wgsl
                targets: &[Some(wgpu::ColorTargetState {                        // tells wgpu what color outputs it should set up. Currently, we only need one for the surface. 
                    format: config.format,                                      // we use the surface's format so that copying to it is easy.
                    blend: Some(wgpu::BlendState::REPLACE),                     // specify that the blending should just replace old pixel data with new data
                    write_mask: wgpu::ColorWrites::ALL,                         // we also tell wgpu to write to all colors: red, blue, green, and alpha.
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {                                   // describes how to interpret our vertices when converting them into triangles.
                topology: wgpu::PrimitiveTopology::TriangleList,                // means that every three vertices will correspond to one triangle.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,                               // FrontFace::Ccw means that a triangle is facing forward if the vertices are arranged in a counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),                              // triangles that are not considered facing forward are culled (not included in the render) as specified by CullMode::Back

                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,

                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,

                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,                                                  // allows wgpu to cache shader compilation data. Only really useful for Android build targets.
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                                                   // determines how many samples the pipeline will use
                mask: !0,                                                   // specifies which samples should be active. In this case, we are using all of them.
                alpha_to_coverage_enabled: false,                           // has to do with anti-aliasing
            },
            multiview: None,                                                // indicates how many array layers the render attachments can have
            cache: None,                                                    // allows wgpu to cache shader compilation data
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
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

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let texture_view_descriptor = wgpu::TextureViewDescriptor::default();
        let view = output.texture.create_view(&texture_view_descriptor);
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        {
            let renderpass_descriptor = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {     // This is what @location(0) in the fragment shader targets
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0, }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            };

            let mut renderpass = encoder.begin_render_pass(&renderpass_descriptor);
            
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.draw(0..3, 0..1);                                            // tell wgpu to draw something with three vertices and one instance. This is where @builtin(vertex_index) comes from.
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}