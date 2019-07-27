use render::window::Window;
use render::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};

fn main() {
    println!("Hello, world!");

    let mut window = Window::new();

    let mut running = true;
    while running {
        window.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match code {
                    VirtualKeyCode::Escape => running = false,
                    _ => {}
                },
                WindowEvent::CloseRequested => running = false,
                _ => {}
            },
            _ => {}
        });
    }
}
