// struct to store the output of our vertex shader.
struct VertexOutput {
    // @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates
    @builtin(position) clip_position: vec4<f32>,

    // Vertical position coordinates passed for the vertex.
    @location(0) vert_pos: vec3<f32>,
};

// @vertex marks that this function as a valid entry point for a vertex shader
@vertex
fn vs_main(
    //  We expect a u32 called in_vertex_index, which gets its value from @builtin(vertex_index).
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    return out;
}

// @fragment marks that this function as a valid entry point for a fragment shader
@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {   // The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}