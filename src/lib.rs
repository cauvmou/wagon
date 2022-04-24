use winit::{event_loop::{self, EventLoop, ControlFlow}, window::WindowBuilder, dpi::PhysicalSize, event::{WindowEvent, VirtualKeyCode}};
mod renderer;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use crate::renderer::Renderer;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {

    // Initialize Wasm logging and hook the default panic handler to output to browser-console.
    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger.");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("Wagon")
        .with_inner_size(PhysicalSize::new(800u32, 450u32))
        .build(&event_loop).expect("Couldn't create Window.");

    // Create Web-canvas and shit
    #[cfg(target_arch="wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-window-hook")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append window to dom.");
    }

    let mut renderer = Renderer::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            winit::event::Event::WindowEvent { window_id, event } => {
                if window_id == window.id() { 
                    match event {
                        WindowEvent::Resized(size) => renderer.resize(size),
                        WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {

                        },
                        WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => renderer.resize(*new_inner_size),
                        _ => {}
                    }
                }
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                renderer.update();
                match renderer.render() {
                    Ok(_) => {},

                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            winit::event::Event::RedrawEventsCleared => {},
            _ => {}
        }
    });
}