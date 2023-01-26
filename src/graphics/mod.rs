use crate::tubes::prelude::{Bowl, Ball};
use winit::window::Window;

pub struct WindowBowl {
    f: Box<dyn Fn(&Window)>,
    win: Window,
    event_loop: winit::event_loop::EventLoop<()>
}

impl Bowl<()> for WindowBowl {
    fn hit(&mut self, _: Ball<()>) {
        (self.f)(&self.win);
    }

    fn type_name() -> String
    where Self: Sized {
        "Window".to_string()
    }
}

impl WindowBowl {
    /// Creates a new `WindowBowl`.
    pub fn new(f: Box<dyn Fn(&Window)>, win_builder: winit::window::WindowBuilder) -> Box<Self> {
        let event_loop = winit::event_loop::EventLoop::new();
        let win = win_builder.build(&event_loop).unwrap();

        Box::new(WindowBowl {
            f,
            win,
            event_loop
        })
    }
    pub fn default(f: Box<dyn Fn(&Window)>) -> Box<Self> {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

        Box::new(WindowBowl {
            f,
            win: window,
            event_loop
        })
    }

    pub fn run(self) {
        self.event_loop.run(move |event,)
    }
}