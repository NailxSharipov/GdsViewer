use log::info;
use winit::event::WindowEvent;
use crate::control::scroll::ScrollControl;
use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(crate) struct ZoomEvent {
    pub(crate) scale: f32,
    pub(crate) cursor: Point,
}

pub(crate) enum NavigationEvent {
    StartZoom(ZoomEvent),
    ProcessZoom(ZoomEvent),
    EndZoom(ZoomEvent),
    CancelZoom(ZoomEvent),
}

pub(crate) struct NavigationControl {
    size: Size,
    cursor: Point,
    scroll_control: ScrollControl,
}

impl NavigationControl {
    pub(crate) fn new() -> Self {
        Self { size: Size { width: 1.0, height: 1.0 }, cursor: Point { x: 0.0, y: 0.0 }, scroll_control: ScrollControl::new() }
    }

    pub(crate) fn update_size(&mut self, size: Size) {
        self.size = size;
        self.cursor = Point { x: 0.5 * size.width, y: 0.5 * size.height };
        self.scroll_control.update_size(size);
    }

    pub(crate) fn process_event(&mut self, event: WindowEvent) -> Option<NavigationEvent> {
        // info!(" {:?}", event);
        match event {
            WindowEvent::MouseWheel { device_id: _, delta, phase } => {
                Some(self.scroll_control.on_scroll(delta, phase, self.cursor))
            }
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.cursor = Point { x: position.x as f32, y: position.y as f32 };
                None
            }
            _ => {
                None
            }
        }
    }
}

