struct VOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VOutput {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-0.9,  0.9),
        vec2<f32>(-0.5,  0.1),
        vec2<f32>( 0.0,  0.5),
        vec2<f32>( 0.3, -0.3),
        vec2<f32>( 0.7,  0.6),
        vec2<f32>( 0.9, -0.7),
    );

    var color = array<vec3<f32>, 6>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(1.0, 0.0, 1.0),
        vec3<f32>(0.0, 1.0, 1.0),
    );

    var out: VOutput;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    out.v_color = vec4<f32>(1.0, 1.0, 0.0, 1.0); // yellow
    return out;
}

@fragment
fn fs_main(input: VOutput) -> @location(0) vec4<f32> {
    return input.v_color;
}
