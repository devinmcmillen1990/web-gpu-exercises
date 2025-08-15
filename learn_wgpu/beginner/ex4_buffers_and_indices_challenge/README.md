# Exercise 4 - Buffers and Indices - Challenge
## Objective
Create a more complex shape than the one we made (aka. more than three triangles) using a vertex buffer and an index buffer. Toggle between the two with the space key

# Solution
Tutorial Source Code - [Learn WGPU - Tutorial 4 Challenge](https://github.com/sotrh/learn-wgpu/blob/master/code/beginner/tutorial4-buffer/src/challenge.rs)

In order to accomplish this, we need to make a few modifications:
  1. Setup buffers to hold the complex shape
  2. Update key board inputs to handle SPACE press
  3. Swap the ex5 pentagon with the complex pentagon

## 1. Setup Buffers to hold the complex shape
From the tutorial, it looks like the complex shape is a 16-point polygon (16-gon) with a green, yellow, red color gradient. Below is the update to ```State::new()``` to setup the buffer data:
```Rust
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
```

Now that we have the triangles and the colors that will makeup our 16-gon, we need to add these into some ```challenge``` buffers.

```Rust
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
```

Now that we have the buffers setup, we need to register them with the Render Pipeline and draw them out when our ```challenge_use_complex``` flag is set to ```true```. Here's the required updates to the ```State::render()``` method:

```Rust
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
```

This is great, but since we defaulted this to ```false```, we'll never actually see our complex shape. To tie in the Space bar, we have to update our ```State::handle_key()``` method.

```Rust
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
```

# Demo
Executing a ```cargo build | cargo run``` will run the application.

Pressing and releasing the space bar will trigger the render pipeline to swap between the shaders.

### Space Bar Released
![alt text](.assets/space_released_final_output.png "Space Bar Released")

### Space Bar Pressed
![alt text](.assets/space_pressed_final_output.png "Space Bar Pressed")