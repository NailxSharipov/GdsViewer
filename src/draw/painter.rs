use wgpu::{Device, Queue, TextureView};
use crate::control::navigation::NavigationEvent;
use crate::draw::geometry::GeometryPainter;
use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(crate) trait Painter {
    fn draw(&mut self, queue: &Queue, device: &Device, view: &TextureView);
    fn update_size(&mut self, size: Size);
    fn update_pos(&mut self, pos: Point);
    fn navigation_event(&mut self, navigation_event: NavigationEvent);
}

pub(crate) enum PainterLibrary {
    Geometry(GeometryPainter)
}

impl Painter for PainterLibrary {
    fn draw(&mut self, queue: &Queue, device: &Device, view: &TextureView) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.draw(queue, device, view);
            }
        }
    }

    fn update_size(&mut self, size: Size) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.update_size(size);
            }
        }
    }

    fn update_pos(&mut self, pos: Point) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.update_pos(pos);
            }
        }
    }

    fn navigation_event(&mut self, navigation_event: NavigationEvent) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.navigation_event(navigation_event);
            }
        }
    }
}