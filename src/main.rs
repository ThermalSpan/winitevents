extern crate glium;

use glium::glutin;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("winit Event printer")
        .with_dimensions(1024, 1024);
    let context = glutin::ContextBuilder::new();
    let _display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut close = false;
    while ! close {
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                println!("{:?}", glutin::WindowEvent::Closed);
                close = true; 
            }
            e => {
                println!("{:?}", e);
            }
        });
    }
}
