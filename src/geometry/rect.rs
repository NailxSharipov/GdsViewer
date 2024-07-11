use crate::geometry::point::Point;
use crate::geometry::size::Size;

pub(crate) struct Rect {
    pub(crate) center: Point,
    pub(crate) size: Size
}

impl Rect {
    pub(crate) fn new(min: Point, max: Point) -> Self {
        let x = 0.5 * (min.x + max.x);
        let y = 0.5 * (min.y + max.y);

        Self {
            center: Point { x, y },
            size: Size { width: max.x - min.x, height: max.y - min.y },
        }
    }

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

    pub(crate) fn max_x(&self) -> f32 {
        self.center.x + 0.5 * self.size.width
    }

    pub(crate) fn max_y(&self) -> f32 {
        self.center.y + 0.5 * self.size.height
    }


}