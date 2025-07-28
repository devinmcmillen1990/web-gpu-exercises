// struct to store the output of the vertex shader.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,    // the @builtin(position) bit tells WGPU that this is the value we want to use as the vertex's clip coordinates
    @location(0) vert_pos: vec3<f32>,               // the @builtin(position) in the fragment shader is the framebuffer space. This means that if your window is 800x600, the x and y of clip_position would be between 0-800 and 0-600, respectively, with the y = 0 being the top of the screen.
};

@vertex                                             // @vertex - marks this function as a valid entry point for a vertex shader
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32     // We expect a u32 called in_vertex_index, which gets its value from @builtin(vertex_index)
) -> VertexOutput {
    var out: VertexOutput;

    // We create two other variables for the x and y of a triangle
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

@fragment                                           // @fragment - marks this function as a valid entry point for a fragment shader.
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {                       // The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target.
    return vec4<f32>(0.3, 0.3, 0.1, 1.0);           // Brown
}
