use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => ()
        }
    }
}