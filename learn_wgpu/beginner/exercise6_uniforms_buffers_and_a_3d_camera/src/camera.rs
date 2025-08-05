use cgmath::SquareMatrix;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

// This matrix will scale and translate our scene from OpenGL's coordinate system to WGPU's
// Note: We don't explicitly need the OPENGL_TO_WGPU_MATRIX, but models centered on (0, 0, 0) will be halfway inside the clipping area. 
//      This is only an issue if you aren't using a camera matrix.
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {

        // this matrix moves the world to be at the position and rotation of the camera 
        //      (essentially an inverse of whatever the transform matrix of the camera would be)
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        // this matrix warps the scene to give the effect of depth. 
        // (without this, objects up close would be the same size as objects far away)
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // The coordinate system in Wgpu is based on DirectX and Metal's coordinate systems
        // That means that in normalized device coordinates, the x-axis and y-axis are in the range of -1.0 to +1.0, 
        //      and the z-axis is 0.0 to +1.0.
        // cgmath crate (as well as most game math crates) is built for OpenGL's coordinate system
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]  // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // Since we can't use cgmath and bytemuck directly we have to convert the Matrix4 into a 4x4 f32 array.
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}