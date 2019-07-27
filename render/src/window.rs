use wgpu::winit::{
    ElementState,
    Event,
    EventsLoop,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};
use wgpu::winit::Window as WinitWindow;

use wgpu::{Surface, Instance};

pub struct Window {
    events_loop: EventsLoop,
    instance: Instance,
    window: WinitWindow,
    surface: Surface
}

impl Window {
    pub fn new() -> Self {
        let mut events_loop = EventsLoop::new();

        let instance = wgpu::Instance::new();

        let window = WinitWindow::new(&events_loop).expect("Expected events_loop");
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        
        let surface = instance.create_surface(&window);

        Window {
            events_loop,
            instance,
            window,
            surface
        }
    }

    pub fn poll_events<F>(&mut self, callback: F) where F: FnMut(Event) {
        self.events_loop.poll_events(callback);

    }
}