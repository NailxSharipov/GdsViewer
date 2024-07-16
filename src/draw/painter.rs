use crate::control::navigation::NavigationEvent;
use crate::draw::context::DrawContext;
use crate::draw::geometry::GeometryPainter;
use crate::geometry::size::Size;

pub(crate) trait Painter {
    fn draw(&mut self, context: &mut DrawContext);
    fn update_size(&mut self, size: Size);
    fn navigation_event(&mut self, navigation_event: NavigationEvent);
}

pub(crate) enum PainterLibrary {
    Geometry(GeometryPainter)
}

impl Painter for PainterLibrary {
    fn draw(&mut self, context: &mut DrawContext) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.draw(context);
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

    fn navigation_event(&mut self, navigation_event: NavigationEvent) {
        match self {
            PainterLibrary::Geometry(painter) => {
                painter.navigation_event(navigation_event);
            }
        }
    }
}