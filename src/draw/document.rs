use i_triangle::delaunay::triangulate::ShapeTriangulate;
use i_triangle::i_overlay::i_float::point::IntPoint;
use i_triangle::i_overlay::i_float::rect::IntRect;
use i_triangle::i_overlay::i_shape::int::shape::IntShapes;
use crate::draw::fill::brush::Brush;
use crate::draw::index_mesh::ListMesh;
use crate::draw::stroke::pencil::Pencil;
use crate::draw::triangulation::PathTriangulation;
use crate::geometry::size::Size;

pub(crate) struct Layer {
    pub(crate) fill_mesh: ListMesh,
    pub(crate) stroke_mesh: ListMesh,
    pub(crate) brush: Brush,
    pub(crate) pencil: Pencil,
    pub(crate) shapes: IntShapes,
    pub(crate) width: f32,
}

pub(crate) struct Document {
    pub(crate) layers: Vec<Layer>,
    pub(crate) size: Size,
}

impl Document {
    pub(crate) fn polygons() -> Self {
        let mut layers = Vec::new();
        let mut rect = IntRect::new(i32::MAX, i32::MIN, i32::MAX, i32::MIN);

        {
            let plus = [
                [
                    IntPoint::new(0, 1),
                    IntPoint::new(0, 2),
                    IntPoint::new(1, 2),
                    IntPoint::new(1, 3),
                    IntPoint::new(2, 3),
                    IntPoint::new(2, 2),
                    IntPoint::new(3, 2),
                    IntPoint::new(3, 1),
                    IntPoint::new(2, 1),
                    IntPoint::new(2, 0),
                    IntPoint::new(1, 0),
                    IntPoint::new(1, 1),
                ].to_vec()
            ].to_vec();

            let tor = [
                [
                    IntPoint::new(5, 0),
                    IntPoint::new(4, 1),
                    IntPoint::new(4, 2),
                    IntPoint::new(5, 3),
                    IntPoint::new(6, 3),
                    IntPoint::new(7, 2),
                    IntPoint::new(7, 1),
                    IntPoint::new(6, 0),
                ].to_vec(),
                [
                    IntPoint::new(5, 1),
                    IntPoint::new(6, 1),
                    IntPoint::new(6, 2),
                    IntPoint::new(5, 2)
                ].to_vec()
            ].to_vec();

            for path in plus.iter() {
                for p in path.iter() {
                    rect.unsafe_add_point(p);
                }
            }

            for path in tor.iter() {
                for p in path.iter() {
                    rect.unsafe_add_point(p);
                }
            }

            let plus_triangulation = plus.triangulation();
            let tor_triangulation = tor.triangulation();

            let mut fill_mesh = ListMesh::with_capacity(plus_triangulation.points.len() + tor_triangulation.points.len());

            fill_mesh.append_triangulation(plus_triangulation);
            fill_mesh.append_triangulation(tor_triangulation);

            layers.push(Layer {
                fill_mesh,
                stroke_mesh: ListMesh { points: vec![], indices: vec![] },
                brush: Brush { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.2 },
                pencil: Pencil {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                shapes: vec![plus, tor],
                width: 0.0,
            }
            );
        }

        {
            let square = [
                [
                    IntPoint::new(2, 0),
                    IntPoint::new(2, 3),
                    IntPoint::new(5, 3),
                    IntPoint::new(5, 0),
                ].to_vec()
            ].to_vec();

            for path in square.iter() {
                for p in path.iter() {
                    rect.unsafe_add_point(p);
                }
            }

            let triangulation = square.triangulation();

            let mut fill_mesh = ListMesh::with_capacity(triangulation.points.len());

            fill_mesh.append_triangulation(triangulation);

            layers.push(Layer {
                fill_mesh,
                stroke_mesh: ListMesh { points: vec![], indices: vec![] },
                brush: Brush { red: 0.0, green: 0.0, blue: 1.0, alpha: 0.2 },
                pencil: Pencil {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                shapes: vec![square],
                width: 0.0,
            });
        }

        Self { layers, size: Size { width: rect.width() as f32, height: rect.height() as f32 } }
    }
}

impl Layer {
    pub(crate) fn build_strokes(&mut self, width: f32) {
        if self.width == width {
            return;
        }
        self.width = width;
        self.stroke_mesh = self.shapes.triangulate_path(width);
    }
}