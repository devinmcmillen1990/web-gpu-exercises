use std::sync::Arc;
use winit::window::Window;

use crate::user_input::UserSelection;

pub struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size:winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    topology: wgpu::PrimitiveTopology,
}

impl State {
    pub async fn new(window: Arc<Window>, selection: UserSelection) -> State {
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let adapater_options = wgpu::RequestAdapterOptions::default();
        let adapter = instance.request_adapter(&adapater_options).await.unwrap();
        let device_descriptor = wgpu::DeviceDescriptor::default();
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];

        let topology = match selection {
            UserSelection::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            UserSelection::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            UserSelection::Help => unreachable!(),
        };
        
        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            topology,
        };

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
        self.configure_surface();
    }

    pub fn render (&mut self) {
        let surface_texture = self.surface.get_current_texture().expect("failed to acquire next swapchain texture");
        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        };
        let texture_view = surface_texture.texture.create_view(&texture_view_descriptor);
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let color_attachment_operations = wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color{ r: 0.5, g: 0.5, b: 0.5, a: 1.0}),
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

        let mut renderpass = encoder.begin_render_pass(&renderpass_descriptor);

        let shader = self.device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu:: FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.topology,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        renderpass.set_pipeline(&pipeline);

        renderpass.draw(0..9, 0..1);

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}