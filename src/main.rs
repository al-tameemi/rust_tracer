mod primitives;
mod objects;
mod shapes;
mod image;

use pixels::{Pixels, SurfaceTexture};
use crate::image::Image;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

const SAMPLES_PER_PIXEL: u32 = 50;
const MAX_DEPTH: u32 = 20;

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let window_size = window.inner_size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()

    };
    // not needed anymore
    drop(window_size);

    let mut image = Image::new(window_size.width, window_size.height as u32, MAX_DEPTH, SAMPLES_PER_PIXEL);

    let mut window_focused = true;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {ref event, window_id} if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    },
                    WindowEvent::Resized(physical_size) => {
                        pixels.resize_surface(physical_size.width, physical_size.height);
                        pixels.resize_buffer(physical_size.width, physical_size.height);
                        image.resize(*physical_size);

                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        pixels.resize_surface(new_inner_size.width, new_inner_size.height);
                        pixels.resize_buffer(new_inner_size.width, new_inner_size.height);
                        image.resize(**new_inner_size);
                    },
                    WindowEvent::Focused(
                        is_focused
                    ) => {
                        window_focused = *is_focused;
                    }
                    _ => {}
                }
            },
            Event::DeviceEvent {ref event, ..} => {
                image.handle_device(event, window_focused);
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                image.clear(pixels.get_frame());
                image.draw(pixels.get_frame());
                pixels.render().unwrap();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}