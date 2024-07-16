use winit::event::{MouseScrollDelta, TouchPhase};
use crate::control::navigation::{NavigationEvent, ZoomEvent};
use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(super) struct ScrollControl {
    scroll: Point,
    sx: f32,
    sy: f32
}

impl ScrollControl {

    pub(super) fn new() -> Self {
        Self { scroll: Point { x: 0.0, y: 0.0 }, sx: 1.0, sy: 1.0 }
    }

    pub(super) fn update_size(&mut self, size: Size) {
        self.sx = 1.0 / size.width;
        self.sy = 1.0 / size.height;
    }

    pub(super) fn on_scroll(&mut self, delta: MouseScrollDelta, phase: TouchPhase, cursor: Point) -> NavigationEvent {
        // info!("scroll: {:?} ", self.scroll);
        match phase {
            TouchPhase::Started => {
                self.scroll = Point { x: 0.0, y: 0.0 };
                self.accumulate(delta);
                let scale = self.value();
                NavigationEvent::StartZoom(ZoomEvent { scale, cursor })
            }
            TouchPhase::Moved => {
                self.accumulate(delta);
                let scale = self.value();
                NavigationEvent::ProcessZoom(ZoomEvent { scale, cursor })
            }
            TouchPhase::Ended => {
                self.accumulate(delta);
                let scale = self.value();
                NavigationEvent::EndZoom(ZoomEvent { scale, cursor })
            }
            TouchPhase::Cancelled => {
                let scale = self.value();
                self.scroll = Point { x: 0.0, y: 0.0 };
                NavigationEvent::CancelZoom(ZoomEvent { scale, cursor })
            }
        }
    }

    fn accumulate(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                let x = self.scroll.x - x;
                let y = self.scroll.y - y;
                self.scroll = Point { x, y };
            }
            MouseScrollDelta::PixelDelta(data) => {
                let x = self.scroll.x - data.x as f32;
                let y = self.scroll.y - data.y as f32;
                self.scroll = Point { x, y };
            }
        }
    }

    fn value(&self) -> f32 {
        (1.0 + self.sy * self.scroll.y).max(0.01)
    }

}