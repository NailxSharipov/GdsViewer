use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(crate) struct Rect {
    pub(crate) center: Point,
    pub(crate) size: Size,
}

impl Rect {
    pub(crate) fn with_size(size: Size) -> Self {
        let x = 0.5 * size.width;
        let y = 0.5 * size.height;

        Self {
            center: Point { x, y },
            size,
        }
    }

    pub(crate) fn min_x(&self) -> f32 {
        self.center.x - 0.5 * self.size.width
    }

    pub(crate) fn min_y(&self) -> f32 {
        self.center.y - 0.5 * self.size.height
    }

    pub(crate) fn scaled(&self, scale: f32) -> Rect {
        Rect { center: self.center, size: self.size.scaled(scale) }
    }
}