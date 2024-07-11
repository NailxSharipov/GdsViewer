use crate::draw::mesh::Mesh;
use crate::draw::r#box::Box;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;

pub(crate) struct Document {
    pub(crate) size: Size,
    pub(crate) mesh: Mesh,
}

impl Document {
    pub(crate) fn five(size: Size) -> Self {
        let mut rects = Vec::new();

        let a = size.width / 6.0;
        let b = size.height / 6.0;

        let size = Size { width: 2.0 * a, height: 2.0 * b };

        // left bottom
        rects.push(Box { rect: Rect { center: Point { x: a, y: b }, size }, brush: 1 });

        // left top
        rects.push(Box { rect: Rect { center: Point { x: a, y: 5.0 * b }, size }, brush: 1 });

        // right top
        rects.push(Box { rect: Rect { center: Point { x: 5.0 * a, y: 5.0 * b }, size }, brush: 1 });

        // right bottom
        rects.push(Box { rect: Rect { center: Point { x: 5.0 * a, y: b }, size }, brush: 1 });


        let mut mesh = Mesh::with_capacity(4 * rects.len());
        for rect in rects {
            mesh.append(rect.mesh());
        }

        Self {
            size,
            mesh,
        }
    }
}