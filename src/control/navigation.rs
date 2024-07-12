use winit::event::{ElementState, MouseButton, WindowEvent};
use crate::control::scroll::ScrollControl;
use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(crate) struct ZoomEvent {
    pub(crate) scale: f32,
    pub(crate) cursor: Point,
}

pub(crate) struct DraggedEvent {
    pub(crate) start: Point,
    pub(crate) current: Point,
}

pub(crate) enum NavigationEvent {
    StartZoom(ZoomEvent),
    ProcessZoom(ZoomEvent),
    EndZoom(ZoomEvent),
    CancelZoom(ZoomEvent),
    StartDragged(DraggedEvent),
    EndDragged(DraggedEvent),
    MoveDragged(DraggedEvent),
}

pub(crate) struct NavigationControl {
    size: Size,
    cursor: Point,
    scroll_control: ScrollControl,
    start_dragged: Point,
    is_left_mouse_pressed: bool,
}

impl NavigationControl {
    pub(crate) fn new() -> Self {
        Self {
            size: Size { width: 1.0, height: 1.0 },
            cursor: Point { x: 0.0, y: 0.0 },
            scroll_control: ScrollControl::new(),
            start_dragged: Point { x: 0.0, y: 0.0 },
            is_left_mouse_pressed: false,
        }
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
                if self.is_left_mouse_pressed {
                    Some(NavigationEvent::MoveDragged(DraggedEvent { start: self.start_dragged, current: self.cursor }))
                } else {
                    None
                }
            }
            WindowEvent::MouseInput { device_id: _, state, button } => {
                if button != MouseButton::Left {
                    return None;
                }

                Some(match state {
                    ElementState::Pressed => {
                        self.start_dragged = self.cursor;
                        self.is_left_mouse_pressed = true;
                        NavigationEvent::StartDragged(DraggedEvent { start: self.start_dragged, current: self.cursor })
                    }
                    ElementState::Released => {
                        self.is_left_mouse_pressed = false;
                        NavigationEvent::EndDragged(DraggedEvent { start: self.start_dragged, current: self.cursor })
                    }
                })
            }
            _ => {
                None
            }
        }
    }
}