use std::sync::{Arc, Mutex};
use log::info;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::app::graphic::GraphicContext;
use crate::draw::painter::Painter;
use crate::geometry::point::Point;

pub struct AppState {
    context: Arc<Mutex<Context>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { context: Arc::new(Mutex::from(Context { state: ContextState::None, graphic: None, counter: 0 })) }
    }

    pub (crate) fn update_pos(&mut self, pos: Point) {
        if let Ok(context) = &mut self.context.lock() {
            if let Some(graphic) = &mut context.graphic {
                graphic.painter_library.update_pos(pos);
            }
        }
    }
}

impl AppState {
    fn context_state(&self) -> ContextState {
        if let Ok(context) = self.context.lock() {
            context.state
        } else {
            ContextState::None
        }
    }
    fn build_window(event_loop: &ActiveEventLoop) -> Window {
        #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowAttributesExtWebSys;
            use wasm_bindgen::JsCast;
            use web_sys::HtmlCanvasElement;

            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let atts = Window::default_attributes().with_canvas(Some(canvas));
            event_loop.create_window(atts).unwrap()
        }

        #[cfg(not(target_arch = "wasm32"))] {
            event_loop.create_window(Window::default_attributes()).unwrap()
        }
    }

    fn init_window(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(mut context) = self.context.lock() {
            context.state = ContextState::Initializing
        }

        let window = Self::build_window(event_loop);

        let clone_context = Arc::clone(&self.context);
        #[cfg(not(target_arch = "wasm32"))] {
            use pollster::FutureExt;
            let graphic = GraphicContext::with_window(window).block_on();
            if let Ok(mut context) = clone_context.lock() {
                context.graphic = Some(graphic);
                context.state = ContextState::Ready;
            };
        }

        #[cfg(target_arch = "wasm32")] {
            use wasm_bindgen_futures::spawn_local;
            spawn_local(async move {
                let graphic = GraphicContext::with_window(window).await;
                if let Ok(mut context) = clone_context.lock() {
                    context.graphic = Some(graphic);
                    context.state = ContextState::Ready;
                };
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextState {
    None,
    Initializing,
    Ready,
}

pub struct Context {
    state: ContextState,
    graphic: Option<GraphicContext>,
    counter: i32,
}

impl Context {
    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        let graphic = if let Some(graphic) = &mut self.graphic {
            graphic
        } else {
            return;
        };

        match event {
            WindowEvent::Resized(new_size) => {
                graphic.resize(new_size.width, new_size.height);
            }
            WindowEvent::RedrawRequested => {
                graphic.draw();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        };
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.context_state() == ContextState::None {
            info!("start init_window");
            self.init_window(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if self.context_state() != ContextState::Ready {
            return;
        }
        if let Ok(context) = &mut self.context.lock() {
            context.handle_window_event(event_loop, event);
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, _event: DeviceEvent) {}

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.context_state() != ContextState::Ready {
            return;
        }

        if let Ok(context) = &mut self.context.lock() {
            if let Some(graphic) = &mut context.graphic {
                graphic.window.request_redraw();
                context.counter += 1;
            }
        }
    }
}
