use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use wgpu::SurfaceError;

pub struct State {
    pub window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            window
        })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {

    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            },
            _ => {}
        }
    }

    // we ask the window to draw another frame as soon as possible as winit only draws one frame unless the window 
    // is resized or we request it to draw another one.
    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.window.request_redraw();
        Ok(())
    }
}