# Exercise 2 - The Surface
* Source Material - [Learn WGPU - The Pipeline](https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#what-s-a-pipeline)
## Concepts
- The Pipeline
- Shaders
# Overview
A pipeline describes all the actions the GPU will perform when acting on a set of data. 

Shaders are mini-programs that you send to the GPU to perform operations on your data.
Types of shaders:
  1. Vertex
  2. Fragment
  3. Compute
  4. Geometry (Not Supported by WebGL)
  5. Tesselation Shaders (Not Supported by WebGL)

Vertex is a point in 2D/3D space. These vertices are then bundled in groups of 2s to form lines and/or 3s to form triangles. Most modern rendering uses triangles to make all shapes, from simple shapes (such as cubes) to complex ones (such as people). These triangles are stored as vertices, which are the points that make up the corners of the triangles.

We use a vertex shader to manipulate the vertices in order to transform the shape to look the way we want it.

The vertices are then converted into fragments. Every pixel in the result image gets at least one fragment. Each fragment has a color that will be copied to its corresponding pixel. The fragment shader decides what color the fragment will be.

[WebGPU Shading Language](https://www.w3.org/TR/WGSL/) (WGSL) is the shader language for WebGPU. WGSL's development focuses on getting it to easily convert into the shader language corresponding to the backend; for example, SPIR-V for Vulkan, MSL for Metal, HLSL for DX12, and GLSL for OpenGL. The conversion is done internally, and we usually don't need to care about the details. In the case of wgpu, it's done by the library called [naga](https://github.com/gfx-rs/naga).
## Objective


## Dependencies