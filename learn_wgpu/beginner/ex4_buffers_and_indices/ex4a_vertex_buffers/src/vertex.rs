#[repr(C)]
#[derive(
    // the Vertex to be Copy so we can create a buffer with it
    Copy,

    Clone,
    
    Debug,

    // Pod indicates that our Vertex is "Plain Old Data", and thus can be interpreted as a &[u8]
    bytemuck::Pod,

    // Zeroable indicates that we can use std::mem::zeroed()
    bytemuck::Zeroable  
)]
pub struct Vertex {
    // represents the x, y, and z of the vertex in 3d space
    position: [f32; 3],

    // represents the red, green, and blue values for the vertex
    color: [f32; 3],
}

// this is the data that will makeup the Randbow Triangle
// we arrange the vertices in counter-clockwise order: top, bottom left, bottom right
// we do it this way because we specified in the primitive of the render_pipeline that we want the front_face 
//      of our triangle to be wgpu::FrontFace::Ccw so that we cull the back face. This means that any triangle 
//      that should be facing us should have its vertices in counter-clockwise order.
pub const VERTICES: &[Vertex] = &[
    Vertex { 
        position: [ 0.0,  0.5, 0.0], 
        color: [1.0, 0.0, 0.0],
    },
    Vertex { 
        position: [-0.5, -0.5, 0.0], 
        color: [0.0, 1.0, 0.0],
    },
    Vertex { 
        position: [ 0.5, -0.5, 0.0], 
        color: [0.0, 0.0, 1.0], 
    },
];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // defines how wide a vertex is
            // when the shader goes to read the next vertex, it will skip over the array_stride number of bytes
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,

            // tells the pipeline whether each element of the array in this buffer represents per-vertex data or per-instance data
            step_mode: wgpu::VertexStepMode::Vertex,

            // describes the individual parts of the vertex
            attributes: &[
                wgpu::VertexAttribute {
                    // defines the offset in bytes until the attribute starts
                    // For the first attribute, the offset is usually zero. For any later attributes, the offset is the sum over size_of of the previous attributes' data.
                    offset: 0,

                    // tells the shader what location to store this attribute at
                    // For example, @location(0) x: vec3<f32> in the vertex shader would correspond to the position field of the Vertex struct, while @location(1) x: vec3<f32> would be the color field
                    shader_location: 0,

                    // tells the shader the shape of the attribute
                    // Float32x3 corresponds to vec3<f32> in shader code
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // defines the offset in bytes until the attribute starts
                    // For the first attribute, the offset is usually zero. For any later attributes, the offset is the sum over size_of of the previous attributes' data.
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,

                    // tells the shader what location to store this attribute at
                    // For example, @location(0) x: vec3<f32> in the vertex shader would correspond to the position field of the Vertex struct, while @location(1) x: vec3<f32> would be the color field
                    shader_location: 1,

                    // tells the shader the shape of the attribute
                    // Float32x3 corresponds to vec3<f32> in shader code
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}