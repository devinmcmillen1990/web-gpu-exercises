use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ KeyEvent, WindowEvent, };
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{ Window, WindowId, };
use wgpu::SurfaceError;

use crate::state::State;

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(window.clone())).unwrap());
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
            },
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {},
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    },
                    Err(e) => {
                        log::error!("Failed to Render: {}", e);
                    },
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                
            },
            // Add empty Mouse Button input events
            WindowEvent::MouseInput { state, button, .. } => match (button, state.is_pressed()) {
                (MouseButton::Left, true) => {}
                (MouseButton::Left, false) => {}
                _ => {}
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => {
                state.handle_key(event_loop, code, key_state.is_pressed());
            },
            _ => {}
        }
    }
}