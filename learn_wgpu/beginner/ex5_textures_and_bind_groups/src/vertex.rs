#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],

    // Updated to take the coordinates for the texture instead of the color
    texture_coords: [f32; 2],
}

pub const VERTICES: &[Vertex] = &[
    // A
    Vertex { position: [-0.0868241, 0.49240386, 0.0], texture_coords: [0.4131759, 0.00759614], },
    // B
    Vertex { position: [-0.49513406, 0.06958647, 0.0], texture_coords: [0.0048659444, 0.43041354], },
    // C
    Vertex { position: [-0.21918549, -0.44939706, 0.0], texture_coords: [0.28081453, 0.949397], },
    // D
    Vertex { position: [0.35966998, -0.3473291, 0.0], texture_coords: [0.85967, 0.84732914], },
    // E
    Vertex { position: [0.44147372, 0.2347359, 0.0], texture_coords: [0.9414737, 0.2652641], },
];

pub const VERTEX_INDICES: &[u16] = &[
    0, 1, 4,        // A, B, E
    1, 2, 4,        // B, C, E
    2, 3, 4,        // C, D, E
];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}