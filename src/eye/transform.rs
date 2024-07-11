use crate::geometry::point::Point;

pub(crate) type Matrix4x4 = [f32; 16];

pub(super) struct OrthoNoRotTransformer {
    pub(super) sx: f32,
    pub(super) sy: f32,
    pub(super) tx: f32,
    pub(super) ty: f32,
}

impl OrthoNoRotTransformer {
    pub(super) fn empty() -> Self {
        Self {
            sx: 1.0,
            sy: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub(super) fn transform(&self, point: Point) -> Point {
        let x = self.sx * point.x + self.tx;
        let y = self.sy * point.y + self.ty;

        Point { x, y }
    }

    pub(super) fn to_matrix(&self) -> Matrix4x4 {
        let s_x = self.sx;
        let s_y = self.sy;
        let t_x = self.tx;
        let t_y = self.ty;
        [
            s_x, 0.0, 0.0, 0.0,
            0.0, s_y, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            t_x, t_y, 0.0, 1.0,
        ]
    }
}