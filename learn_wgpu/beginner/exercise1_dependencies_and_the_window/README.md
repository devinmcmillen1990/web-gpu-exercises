# Exercise 1 - Dependencies and the Window
(Some overview about the exercise)
## Objective

## Concepts
- Dependencies
- The Window
- User Input
  - Press the ESCAPE key to close the window.

## Dependencies
Here are the dependencies in the Cargo.toml:
```rust
[dependencies]
anyhow = "1.0"
winit = { version = "0.30", features = ["android-native-activity"] }
env_logger = "0.10"
log = "0.4"
wgpu = "25.0"
pollster = "0.3"
```