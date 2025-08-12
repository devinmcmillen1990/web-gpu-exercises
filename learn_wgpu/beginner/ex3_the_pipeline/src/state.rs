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

    // Maintaining Render Pipeline in State now
    render_pipeline: wgpu::RenderPipeline,
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
        let surface_capabilities = surface.get_capabilities(&adapter);
        let size = window.inner_size();
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

        // This will load the Vertex and Fragment shaders in the shader.wgsl file.
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Now that we have the shaders loaded, we need to incorporate them into a Pipeline Layout to be loaded into the Render Pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create the Render Pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),

            // Attach the Render Pipeline Layout to this Render Pipeline
            layout: Some(&render_pipeline_layout),

            // Used for storing the vertices to the surface
            vertex: wgpu::VertexState {
                // Attach the loaded shader to the Vertex State.
                module: &shader,

                // Shader Vertex entry point (function)
                // @vertex decorated function in shader.wgsl
                entry_point: Some("vs_main"),

                // tells wgpu what type of vertices we want to pass to the vertex shader 
                // we're specifying the vertices in the vertex shader itself, so we'll leave this empty.
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },

            // Used for storing the colors to the surface
            fragment: Some(wgpu::FragmentState {
                // Attach the loaded shader to the Fragment State.
                module: &shader,

                // Shader Fragment entry point
                // @fragment decorated function in shader.wgsl
                entry_point: Some("fs_main"),

                // tells wgpu what color outputs it should set up. Currently, we only need one for the surface. 
                targets: &[Some(wgpu::ColorTargetState {
                    // use the surface's format
                    format: config.format,

                    // specify that blending should replace old pixel data
                    blend: Some(wgpu::BlendState::REPLACE),

                    // tells wgpu to write all colors (red, blue, green, alpha).
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            
            // tells the wgpu how to interpret our vertices when converting them in to the desired topology
            primitive: wgpu::PrimitiveState {

                // means that every 3 vertices will correspond to 1 triangle
                topology: wgpu::PrimitiveTopology::TriangleList,
                
                strip_index_format: None,

                // FrontFace::Ccw means that a triangle is facing forward if the vertices are arrange in a counter-clockwise direction
                front_face: wgpu::FrontFace::Ccw,

                // triangles that are not considered facing forward are culled (not included in the render) as specified by CullMode::Back
                cull_mode: Some(wgpu::Face::Back),

                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,

                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,

                // allows wgpu to cache shader compilation data. Only really useful for Android build targets.
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                // determines how many samples the pipeline will use
                count: 1,

                // specifies which samples should be active. In this case, we are using all of them.
                mask: !0,

                // has to do with anti-aliasing
                alpha_to_coverage_enabled: false,
            },

            // indicates how many array layers the render attachments can have
            multiview: None,

            // allows wgpu to cache shader compilation data
            cache: None,
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
        match (code, is_pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            },
            _ => {}
        }
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
            let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0, }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })
                ],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Attach the render pipeline to the RenderPass
            renderpass.set_pipeline(&self.render_pipeline);

            // Tell wgpu to draw
            renderpass.draw(
                0..3,           // This is 3 vertices (the 3 in the shader.wgsl)
                0..1            // this means to crate only 1 instance (This is where @builtin(vertex_index) comes from)
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}