use crate::geometry::point::Point;

pub(crate) struct Mesh {
    pub(crate) points: Vec<Point>,
    pub(crate) brushes: Vec<u32>,
    pub(crate) indices: Vec<u32>, // we will use clockwise order for a face side
}

impl Mesh {

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self { points: Vec::with_capacity(capacity), brushes: Vec::with_capacity(capacity), indices: Vec::with_capacity(3 * capacity) }
    }

    pub(crate) fn append(&mut self, mut mesh: Mesh) {
        let offset = self.points.len() as u32;

        self.points.append(&mut mesh.points);
        self.brushes.append(&mut mesh.brushes);

        let mut indices = mesh.indices;
        for i in indices.iter_mut() {
            *i += offset;
        }

        self.indices.append(&mut indices);
    }
}