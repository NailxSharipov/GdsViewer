use wgpu::{Device, Queue, TextureView};
use crate::draw::geometry::GeometryPainter;
use crate::geometry::point::Point;

pub(crate) trait Painter {
    fn draw(&mut self, queue: &Queue, device: &Device, view: &TextureView);
    fn update_size(&mut self, screen_width: u32, screen_height: u32);
    fn update_scale(&mut self, scale: f32);
    fn update_pos(&mut self, pos: Point);
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

    fn update_size(&mut self, screen_width: u32, screen_height: u32) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.update_size(screen_width, screen_height);
            }
        }
    }

    fn update_scale(&mut self, scale: f32) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.update_scale(scale);
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
}