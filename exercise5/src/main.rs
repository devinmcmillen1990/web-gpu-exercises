mod state;
mod user_input;

use crate::state::State;
use crate::user_input::{UserSelection, parse_user_input};

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId}
};

#[derive(Default)]
struct App {
    state: Option<State>,
    user_selection: UserSelection,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let user_selection = self.user_selection;
        let state = pollster::block_on(State::new(window.clone(), user_selection));
        self.state = Some(state);
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("Close Button Pressed; Stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            _ => (),
        }
    }
}

pub fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let user_input_selection = parse_user_input();

    let mut app = App {
        user_selection: user_input_selection,
        ..Default::default()
    };

    event_loop.run_app(&mut app).unwrap();
}