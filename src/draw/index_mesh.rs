use i_triangle::triangulation::int::Triangulation;

pub(crate) struct ListMesh {
    pub(crate) points: Vec<[f32; 2]>,
    pub(crate) indices: Vec<u32>,
}

impl ListMesh {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self { points: Vec::with_capacity(capacity), indices: Vec::with_capacity(3 * capacity) }
    }

    pub(crate) fn append_triangulation(&mut self, triangulation: Triangulation) {
        let offset = self.points.len() as u32;

        self.points.extend(triangulation.points.iter().map(|p| [p.x as f32, p.y as f32]));
        self.indices.extend(triangulation.indices.iter().map(|&i| i as u32 + offset ));
    }
}