use crate::draw::mesh::Mesh;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;

pub(crate) struct Box {
    pub(crate) rect: Rect,
    pub(crate) brush: u32, // index to brush
}

impl Box {
    pub(crate) fn mesh(&self) -> Mesh {
        let points = self.points().to_vec();

        let indices = vec![
            0, 3, 1, 2, 1, 3,
        ];
        let brushes = vec![self.brush; 4];
        Mesh { points, brushes, indices }
    }

    #[inline(always)]
    fn points(&self) -> [Point; 4] {
        let x0 = self.rect.min_x();
        let x1 = self.rect.max_x();
        let y0 = self.rect.min_y();
        let y1 = self.rect.max_y();

        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x0, y: y1 };
        let p2 = Point { x: x1, y: y1 };
        let p3 = Point { x: x1, y: y0 };

        [p0, p1, p2, p3]
    }
}