# Exercise 5 - Triangle List and Strip Primitives
## Dependencies
- wgpu
- winit
- env_logger
- bytemuck
- pollster
## Objective
The objective of this exercise is to explore WGPU TriangleList and TriangleStrip functionality, while providing the user the ability to modify the topology via the command line.
## Key Concepts
- WebGPU Point/Line Primitives
  1. Triangle-List  - Collection of triple-verticies representing triangles a location in space.
  3. Triangle-Strip - Connect triangles sequentially from a base triangle (creating the next triangle sequence by adding a vertex to the closest edge of the last generated triangle) to form a single, continuous polyline.
## Output
1. Triangle-List
- ```cargo run triangle-list```
![alt text](.assets/triangle-list-output.png "Triangle List Output")
  * NOTE: This image isn't super valuable because the points are the size of pixels. They may not be visible depending on the resolution.
2. Triangle-Strip
- ```cargo run triangle-strip```
![alt text](.assets/triangle-strip-output.png "Triangle Strip Output")
## Code Notes
```rust
// src.state.rs - lina 142
// Update renderpass.draw() to include the 9 vertexes.
renderpass.draw(0..9, 0..1);
```