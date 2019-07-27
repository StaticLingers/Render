#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn test() {
    println!("Hello render");
}

pub mod window;

pub use wgpu::winit::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};