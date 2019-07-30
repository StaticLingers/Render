use wgpu::winit::{
    ElementState,
    Event as WinitEvent,
    EventsLoop,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};
use wgpu::winit::Window as WinitWindow;

use wgpu::{Surface, Instance};

pub use wgpu::winit::VirtualKeyCode as KeyCode;

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

    pub fn poll_events<F>(&mut self, mut callback: F) where F: FnMut(Event) {
        self.events_loop.poll_events(|event| {
            callback(Event::from(event))
        });

    }
}

#[derive(Debug)]
pub enum Event {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    Close,
    Unknown
}

impl From<WinitEvent> for Event {
    fn from(item: WinitEvent) -> Event {
        match item {
            WinitEvent::WindowEvent {event, ..} => match event {
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(code),
                        state,
                        ..
                    },
                    ..
                } => match state {
                    ElementState::Pressed => Event::KeyPressed(code),
                    ElementState::Released => Event::KeyReleased(code)
                },
                WindowEvent::CloseRequested => Event::Close,
                _ => Event::Unknown
            },
            _ => Event::Unknown
        }
    }

}
