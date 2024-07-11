use log::info;
use winit::event_loop::EventLoop;
use gds_viewer::app::state::AppState;

pub fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Set log level to Info
        .init();
    info!("Starting application...");
    let event_loop = EventLoop::new().unwrap();
    let mut state = AppState::new();
    let _ = event_loop.run_app(&mut state);
    info!("Release application...");
}
