# Exercise 3 - Color Triangle
## Dependencies
- wgpu
- winit
- env_logger
- bytemuck
- pollster
## Objective
Use WGPU and WINIT to generate a multi-colored triangle exploring shaders.
## Output
![alt text](.assets/colorful_triangle.png "Colorful Triangle")
## Project Notes
- Introduced state.rs to help manage window rendering for the ApplicationHandler.
- Introduced shader.wgsl to start with vertex and fragment shading.
## Code Notes
1. Background color gray
```rust
// src/state.rs - line 84
let color_attachment_operations = wgpu::Operations {
    load: wgpu::LoadOp::Clear(wgpu::Color{ 
        r: 0.5, 
        g:0.5, 
        b: 0.5, 
        a: 1.0,}), // gray
        store: wgpu::StoreOp::Store,
};
```