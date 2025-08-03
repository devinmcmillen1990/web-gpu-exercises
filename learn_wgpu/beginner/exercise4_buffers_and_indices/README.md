# Exercise 4 - Buffers and Indices
* Source Material - [Learn WGPU - Buffers and Indices](https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#we-re-finally-talking-about-them)
## Concepts
- Buffers (Vertex and Index)
- DeviceExt
- bytemuck
# Overview
## Buffers
  - Buffer is a blob of contiguous memory on the GPU.
  - Generally used for storing data in simple data structures like structs or arrays.
    - Can also be used to store data in more complext data structures like graphs and trees, provided all the nodes are stored together and don't reference data that is outside of the buffer.
## Objective

## Dependencies