use crate::draw::color::{Color, COLORS};
use crate::geometry::point::Point;

pub(crate) const MAX_BRUSHES: usize = 16;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Brush {
    pub(crate) width: f32,
    pub(crate) vec: Point,
    pub(crate) color: Color,
}

impl Brush {
    pub(crate) fn create_set() -> [Brush; MAX_BRUSHES] {
        let mut brushes = [Brush {
            width: 0.0,
            vec: Point { x: 0.0, y: 0.0 },
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
        }; MAX_BRUSHES];

        let a = 1.0 / 2.0_f32.sqrt();

        let vecs = [
            Point { x: a, y: a },
            Point { x: a, y: -a },
        ];

        for i in 0..MAX_BRUSHES {
            brushes[i] = Brush {
                width: 1.0,
                vec: vecs[i % vecs.len()],
                color: COLORS[i],
            }
        }

        brushes
    }
}