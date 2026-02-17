use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct GameWindow {
    pub window: Window,
    pub width: u32,
    pub height: u32,
}

impl GameWindow {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
                    .with_title("3D Pac-Man"),
            )
            .expect("Failed to create window");

        let width = window.inner_size().width;
        let height = window.inner_size().height;

        Self {
            window,
            width,
            height,
        }
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

pub fn create_event_loop() -> EventLoop<()> {
    EventLoop::new().expect("Failed to create event loop")
}
