use i_triangle::i_overlay::i_shape::int::path::IntPath;
use i_triangle::i_overlay::i_shape::int::shape::{IntShape, IntShapes};
use crate::draw::index_mesh::ListMesh;
use crate::geometry::point::Point;

pub(crate) trait PathTriangulation {
    fn triangulate_path(&self, width: f32) -> ListMesh;
}

pub(crate) trait PolygonSize {
    fn vertices_count(&self) -> usize;
}

impl PathTriangulation for IntPath {
    fn triangulate_path(&self, width: f32) -> ListMesh {
        let n = self.vertices_count();
        let mut points = Vec::with_capacity(4 * n);
        let mut indices = Vec::with_capacity(6 * n);
        let r = 0.5 * width;

        feed(&mut points, &mut indices, self, r);

        ListMesh { points, indices }
    }
}

impl PathTriangulation for IntShape {
    fn triangulate_path(&self, width: f32) -> ListMesh {
        let n = self.vertices_count();
        let mut points = Vec::with_capacity(4 * n);
        let mut indices = Vec::with_capacity(6 * n);
        let r = 0.5 * width;

        for path in self {
            feed(&mut points, &mut indices, path, r);
        }

        ListMesh { points, indices }
    }
}

impl PathTriangulation for IntShapes {
    fn triangulate_path(&self, width: f32) -> ListMesh {
        let n = self.vertices_count();
        let mut points = Vec::with_capacity(4 * n);
        let mut indices = Vec::with_capacity(6 * n);
        let r = 0.5 * width;

        for shape in self {
            for path in shape {
                feed(&mut points, &mut indices, path, r);
            }
        }

        ListMesh { points, indices }
    }
}

fn feed(points: &mut Vec<[f32; 2]>, indices: &mut Vec<u32>, path: &IntPath, r: f32) {
    let mut a = Point::with_int_point(&path[path.len() - 1]);
    let mut n = points.len() as u32;
    for p in path.iter() {
        let b = Point::with_int_point(p);
        let t = (b - a).tangent(r);

        let p0 = a - t;
        let p1 = a + t;
        let p2 = b + t;
        let p3 = b - t;

        points.push([p0.x, p0.y]);
        points.push([p1.x, p1.y]);
        points.push([p2.x, p2.y]);
        points.push([p3.x, p3.y]);


        indices.push(n);
        indices.push(n + 2);
        indices.push(n + 1);
        indices.push(n);
        indices.push(n + 3);
        indices.push(n + 2);

        a = b;
        n += 4;
    }
}

impl PolygonSize for IntPath {
    fn vertices_count(&self) -> usize {
        self.len()
    }
}

impl PolygonSize for IntShape {
    fn vertices_count(&self) -> usize {
        let mut n = 0;
        for path in self.iter() {
            n += path.vertices_count();
        }

        n
    }
}

impl PolygonSize for IntShapes {
    fn vertices_count(&self) -> usize {
        let mut n = 0;
        for shape in self.iter() {
            n += shape.vertices_count();
        }

        n
    }
}