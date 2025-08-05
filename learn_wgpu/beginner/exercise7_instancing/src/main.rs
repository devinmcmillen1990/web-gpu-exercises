mod app;
mod camera;
mod instance;
mod state;
mod texture;
mod vertex;

use winit::event_loop::{EventLoop, ControlFlow, };

use crate::app::App;

pub fn main() {
    let _ = run();
}

fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

    Ok(())
}