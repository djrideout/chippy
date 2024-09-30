use crate::frontend::Core;
use error_iter::ErrorIter as _;
use log::error;
use std::sync::{Arc, Mutex};
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct Display {
    core: Arc<Mutex<dyn Core>>,
    width: usize,
    height: usize,
    keymap: Arc<Mutex<[VirtualKeyCode]>>,
}

impl Display {
    pub fn new(core: Arc<Mutex<impl Core>>, width: usize, height: usize, keymap: Arc<Mutex<[VirtualKeyCode]>>) -> Display {
        Display {
            core,
            width,
            height,
            keymap
        }
    }

    pub async fn run(&self) {
        // Set up graphics buffer and window
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(self.width as f64, self.height as f64);
            WindowBuilder::new()
                .with_title("chippy")
                .with_inner_size(size.to_physical::<f64>(5.0))
                .with_min_inner_size(size)
                .build(&event_loop)
                .expect("WindowBuilder error")
        };

        let window = Rc::new(window);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowExtWebSys;
    
            // Retrieve current width and height dimensions of browser client window
            let get_window_size = || {
                let client_window = web_sys::window().unwrap();
                LogicalSize::new(
                    client_window.inner_width().unwrap().as_f64().unwrap(),
                    client_window.inner_height().unwrap().as_f64().unwrap(),
                )
            };
    
            let window = Rc::clone(&window);
    
            // Initialize winit window with current dimensions of browser client
            window.set_inner_size(get_window_size());
    
            let client_window = web_sys::window().unwrap();
    
            // Attach winit canvas to body element
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
    
            // Listen for resize event on browser client. Adjust winit window dimensions
            // on event trigger
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                let size = get_window_size();
                window.set_inner_size(size)
            }) as Box<dyn FnMut(_)>);
            client_window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
            Pixels::new_async(self.width as u32, self.height as u32, surface_texture).await.expect("Pixels error")
        };

        let core = self.core.clone();
        let keymap = self.keymap.clone();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                let mut core = core.lock().unwrap();
                core.draw(pixels.frame_mut());
                drop(core);
                if let Err(err) = pixels.render() {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
    
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
    
                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
    
                let mut core = core.lock().unwrap();
                // Handle key presses
                let keymap = keymap.lock().unwrap();
                for i in 0 .. keymap.len() {
                    if input.key_released(keymap[i]) {
                        core.release_key(i);
                    } else if input.key_pressed(keymap[i]) || input.key_held(keymap[i]) {
                        core.press_key(i);
                    } else {
                        core.release_key(i);
                    }
                }
                drop(core);
    
                // Update internal state and request a redraw
                window.request_redraw();
            }
        })
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
