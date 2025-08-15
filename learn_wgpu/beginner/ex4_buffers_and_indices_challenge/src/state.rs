use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use wgpu::SurfaceError;
use wgpu::util::DeviceExt;

use crate::vertex::{ Vertex, VERTICES, VERTEX_INDICES, };

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    // Storing separate instances of the challenge buffers so that we can swap them during rendering
    challenge_vertex_buffer: wgpu::Buffer,
    challenge_index_buffer: wgpu::Buffer,
    challenge_num_indices: u32,

    // flag used to determine whether or not to render the complex challenge shape
    challenge_use_complex: bool,
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
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Index Buffer"),
            contents: bytemuck::cast_slice(VERTEX_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // represents the points spaced uniformly around a circle (16-gon)
        let num_vertices = 16;

        // one full turn is 2π radians, so the step between consecutive vertices is Δ𝜃 = 2𝜋/𝑁
        // then, for the i-th vertex, θ_i = i ⋅ Δθ
        let angle = std::f32::consts::PI * 2.0 / num_vertices as f32;
        
        // Polar -> Cartesian
        let challenge_vertices = (0..num_vertices).map(|i| {
            // for the i-th vertex, <theta>θ_i = i ⋅ Δθ
            let theta = angle * i as f32;

            // A point on a circle with radius "r" that is centered at the origin is: (x = r ⋅ cos(θ), y = r ⋅ sin(θ))
            // Here r = 0.5. That keeps the polygon safely inside the clip-space unit square (NDC is [−1,1] in both x and y).
            let circle_radius = 0.5;

            Vertex {
                position: [
                    circle_radius * theta.cos(),

                    // the negative sign will flip the y-axis
                    // this is like reflecting the polygon across the x-axis, which will affect the clockwise vs. counter-clockwise orientation that will matter for backface culling
                    -(circle_radius) * theta.sin(),
                    0.0
                ],
                // remember that cos and sin produce values in the range [−1,1]. In order to get the range [0,1] we can apply the formula (x + 1)/2.
                // this will apply a smooth angular color gradient around the shape
                // interpolation across triangles creates the continuous “mango” look
                color: [
                    (1.0 + theta.cos()) / 2.0,
                    (1.0 + theta.sin()) / 2.0,
                    0.0
                ]
            }
        }).collect::<Vec<_>>();

        // A convex polygon with 𝑁 vertices can be broken into exactly N−2 triangles when you fan from a single vertex (vertex 0)
        //      EX: Pentagon (5 Vertices) -> 3 Triangles
        //      EX: Hexagon  (6 Vertices) -> 4 Triangles
        // we anchor at vertex 0 and connect consecutive outer vertices
        // because y uses -sin(θ) (clockwise ordering), we flip the last two vertices to keep CCW winding under back-face culling
        let num_triangles = num_vertices - 2;

        let challenge_indices = (1u16..num_triangles + 1)
            // one CCW triangle per i (given y = -sin(θ))
            .flat_map(|i| {
                // represents one triangle, stored in index order for correct winding.
                [0, i + 1, i]
            })
            .collect::<Vec<_>>();

        let challenge_num_indices = challenge_indices.len() as u32;

        // create the challenge vertex buffer
        let challenge_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Challenge Vertex Buffer"),
            contents: bytemuck::cast_slice(&challenge_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // create the challenge vertex index buffer
        let challenge_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Challenge Index Buffer"),
            contents: bytemuck::cast_slice(&challenge_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // default to show pentagon when SPACE is not pressed
        let challenge_use_complex = false;

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
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
            vertex_buffer,
            index_buffer,
            num_indices: VERTEX_INDICES.len() as u32,

            // Register with the state to use during rendering
            challenge_vertex_buffer,
            challenge_index_buffer,
            challenge_num_indices,
            challenge_use_complex,
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

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            // Add block for handling SPACE bar press
            (KeyCode::Space, pressed) => {
                // toggle challenge_use_complex flag that will swap the vertex buffers during rendering
                self.challenge_use_complex = pressed;
            },
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
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
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
            });

            renderpass.set_pipeline(&self.render_pipeline);

            // This will swap the buffers depending on the status of "challenge_use_complex"
            let data = if self.challenge_use_complex {
                (
                    &self.challenge_vertex_buffer,
                    &self.challenge_index_buffer,
                    self.challenge_num_indices,
                )
            } else {
                (
                    &self.vertex_buffer,
                    &self.index_buffer,
                    self.num_indices,
                )
            };

            // apply the dynamic buffers
            renderpass.set_vertex_buffer(0, data.0.slice(..));
            renderpass.set_index_buffer(data.1.slice(..), wgpu::IndexFormat::Uint16);
            renderpass.draw_indexed(0..data.2, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}