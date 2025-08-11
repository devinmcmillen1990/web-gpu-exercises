use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent, };
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId, };
use wgpu::SurfaceError;

use crate::state::State;

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

// ApplicationHander will give us a variety of different functions that we can use to get application events such 
// as key press, mouse movements and various lifecycle events.
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Used to hold attributes about the window (using default for now)
        let window_attributes = Window::default_attributes();

        // Represents the window that we will be creating in this exercise
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Set the state
        self.state = Some(pollster::block_on(State::new(window.clone())).unwrap());

        // Signals to the event loop that the window's content needs to be updated
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        // for window events see https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html
        match event {
            // The window has been requested to close.
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },

            // The size of the window has changed
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
            },

            // Emitted when a window should be redrawn
            WindowEvent::RedrawRequested => {
                match state.render() {
                    // Ok -> Nothing to do
                    Ok(_) => {},

                    // 
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    },

                    // Handle other errors -> Unable to render
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            },

            // Handles Keyboard Input Events
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