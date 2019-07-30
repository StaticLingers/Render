use render::window::Window;
use render::{window::Event, window::KeyCode};

fn main() {
    println!("Hello, world!");

    let mut window = Window::new();

    let mut running = true;
    while running {
        window.poll_events(|event| 
            match event {
                Event::KeyReleased(KeyCode::Escape) | 
                Event::Close => running = false,
                _ => {}
            
        });

        window.draw();
    }
}
