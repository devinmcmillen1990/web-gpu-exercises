use std::sync::Arc;
use image::GenericImageView;
use wgpu::util::DeviceExt;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;

use crate::vertex::{Vertex, VERTICES, INDICES, };

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
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

        let diffuse_bytes = include_bytes!("../.assets/happy-tree.png");                  // grab the bytes for the image
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();    // load the bytes into an image
        let diffuse_rgba = diffuse_image.to_rgba8();                            // convert it to a vec of rgba bytes
        let dimensions = diffuse_image.dimensions();                            // store dimensions for when we create the actual Texture

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,                                           // all textures are stored as 3D. To represent a 2D texture, we set depth to 1
        };

        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,                    // Most images are stored using sRGB, so we need to reflect that here.
                usage: 
                    wgpu::TextureUsages::TEXTURE_BINDING                        // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                    | wgpu::TextureUsages::COPY_DST,                            // COPY_DST means that we want to copy data to this texture
                label: Some("diffuse_texture"),
                view_formats: &[],                                              // specifies what texture formats can be used to create TextureViews for this texture. The base texture format (Rgba8UnormSrgb in this case) is always supported
            }
        );

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {                // Tells wgpu where to copy the pixel data
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,                              // The actual pixel data
            wgpu::TexelCopyBufferLayout {               // The layout of the texture
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let diffuse_texture_view_descriptor = wgpu::TextureViewDescriptor::default();
        let diffuse_texture_view = diffuse_texture.create_view(&diffuse_texture_view_descriptor);        
        let sampler_descriptor = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,             // address_mode_* parameters determine what to do if the sampler gets a texture coordinate that's outside the texture itself
            address_mode_v: wgpu::AddressMode::ClampToEdge,             // address_mode_* parameters determine what to do if the sampler gets a texture coordinate that's outside the texture itself
            address_mode_w: wgpu::AddressMode::ClampToEdge,             // address_mode_* parameters determine what to do if the sampler gets a texture coordinate that's outside the texture itself
            mag_filter: wgpu::FilterMode::Linear,                       // describe what to do when the sample footprint is larger than one texel
            min_filter: wgpu::FilterMode::Nearest,                      // describe what to do when the sample footprint is smaller than one texel
            mipmap_filter: wgpu::FilterMode::Nearest,                   // similar to (mag/min)_filter as it tells the sampler how to blend between mipmaps
            .. Default::default()
        };
        let diffuse_sampler = device.create_sampler(&sampler_descriptor);

        let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,                               // Visible only to the Fragment Shader
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true, }
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,                               // Visible only to the Fragment Shader
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),    // This should match the filterable field of the corresponding Texture entry above.
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),   
        };

        let texture_bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

        let diffuse_bind_group_descriptor = wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        };
        let diffuse_bind_group = device.create_bind_group(&diffuse_bind_group_descriptor);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        };
        let render_pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
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
            num_vertices: VERTICES.len() as u32,
            index_buffer,
            num_indices: INDICES.len() as u32,
            diffuse_bind_group,
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
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(
                            wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0, }
                        ),
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

            // NEW
            renderpass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            renderpass.draw_indexed(0..self.num_indices, 0, 0..1);
            renderpass.draw(0..self.num_vertices, 0..1);
            renderpass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}