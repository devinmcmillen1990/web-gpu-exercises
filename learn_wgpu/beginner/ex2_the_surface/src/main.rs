mod app;
mod state;

use winit::event_loop::{ EventLoop, ControlFlow, };

use crate::app::App;

pub fn main() {
    env_logger::init();

    if let Err(e) = run() {
        log::error!("Fatal error: {}", e);
    }
}

fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

    Ok(())
}