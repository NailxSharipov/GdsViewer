use std::panic;
use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
use winit::event_loop::EventLoop;
use crate::app::state::AppState;

#[wasm_bindgen]
pub struct App {
    state: AppState
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn create() -> Self {
        Self { state: AppState::new() }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("error initializing log");
        info!("Starting application...");
        let event_loop = EventLoop::new().unwrap();
        match event_loop.run_app(&mut self.state) {
            Ok(_) => {
                info!("Run application...");
            }
            Err(err) => {
                info!("err application {}", err);
            }
        }
        info!("Release application...");
    }
}