# Exercise 4b - Index Buffers
* Source Material - [Learn WGPU - Textures and Bind Groups](https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/)
## Objective

## Concepts
- Textures
- Image File Loading
# Overview
## Textures
Textures are images overlaid on a triangle mesh to make it seem more detailed.

There are multiple types of textures, such as:
- normal maps,
- bump maps,
- specular maps, and
- diffuse maps

# Dependencies
```rust
[dependencies]
anyhow = "1.0"
winit = "0.30.12"
env_logger = "0.11.8"
log = "0.4"
wgpu = "26.0.1"
pollster = "0.4.0"
bytemuck = "1.23.1"
```
# Output