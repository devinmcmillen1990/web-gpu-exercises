mod app;
mod utils;
use winit::event_loop::{EventLoop,ControlFlow};

use crate::app::App;

pub fn main() {
    // utils::list_gpus::list_gpus();
    
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}