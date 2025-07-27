use std::sync::Arc;
use winit::window::Window;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            window,
        })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        
    }

    pub fn render(&mut self) {
        // Ask the window to redraw the frame as soon as possible, as winit only draws one frame unless the window is 
        // resized or we request it to draw another one.
        self.window.request_redraw();
    }
}