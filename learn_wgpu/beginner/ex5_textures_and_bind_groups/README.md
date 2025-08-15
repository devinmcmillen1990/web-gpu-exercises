# Exercise 3 - The Pipeline
Tutorial Link - [Learn WGPU - Textures and Buffers](https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures)

Tutorial Source Code - [Learn WGPU - Tutorial 5](https://github.com/sotrh/learn-wgpu/tree/master/code/beginner/tutorial5-textures/)

## Textures
<strong>Textures</strong> are images overlaid on a triangle mesh. There are multiple types of textures, including:
  - normal maps
  - bump maps
  - specular maps
  - diffuse maps



## Bind Groups

## Demo
Executing a ```cargo build | cargo run``` will run the application rendering a blueish screen the Happy Tree in on a pentagon.
![alt text](.assets/ex5_final_output.png "Demo Final Output - Textures and Buffers")

Keybindings are the same from the previous exercise.

## Challenge
Create a second pipeline that uses the triangle's position data to create a color that it then sends to the fragment shader. Have the app swap between these when you press the spacebar. Hint: you'll need to modify VertexOutput

[EX5 - Textures and Bind Groups - Challenge](../ex5_textures_and_bind_groups_challenge/README.md)