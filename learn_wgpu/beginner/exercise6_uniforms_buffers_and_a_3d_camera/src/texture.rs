use image::GenericImageView;
use anyhow::*;

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let texture_image = image::load_from_memory(bytes)?;
        Self::from_image(&device, &queue, &texture_image, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_image: &image::DynamicImage,
        label: Option<&str>, 
    ) -> Result<Self> {
        let rgba = texture_image.to_rgba8();
        let dimensions = texture_image.dimensions();
        let texture = Self::create_texture(&device, &queue, &rgba, dimensions, label);
        let view = Self::create_view(&texture);
        let sampler = Self::create_sampler(&device);

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    fn create_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        dimensions: (u32, u32),
        label: Option<&str>,
    ) -> wgpu::Texture {
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = device.create_texture(&texture_descriptor);

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        texture
    }

    fn create_view(
        texture: &wgpu::Texture
    ) -> wgpu::TextureView {
        let texture_view_descriptor = wgpu::TextureViewDescriptor::default();
        
        let view = texture.create_view(&texture_view_descriptor);

        view
    }

    fn create_sampler(
        device: &wgpu::Device,
    ) -> wgpu::Sampler {
        let sampler_descriptor = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };

        let sampler = device.create_sampler(&sampler_descriptor);

        sampler
    }
}