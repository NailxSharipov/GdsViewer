use crate::eye::transform::{Matrix4x4, OrthoNoRotTransformer};
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;

pub(crate) struct OrthoNoRotCamera {
    screen: Size,
    view_box: Rect,
    zoom: f32,
    screen_to_world: OrthoNoRotTransformer,
    world_to_clip: OrthoNoRotTransformer,
    timestamp: usize,
}

impl OrthoNoRotCamera {
    pub(crate) fn new(screen: Size, view_box: Rect) -> Self {
        let mut camera = Self {
            screen,
            view_box,
            zoom: 1.0,
            screen_to_world: OrthoNoRotTransformer::empty(),
            world_to_clip: OrthoNoRotTransformer::empty(),
            timestamp: 0,
        };

        camera.update();

        camera
    }

    pub(crate) fn timestamp(&self) -> usize {
        self.timestamp
    }

    pub(crate) fn set_screen(&mut self, screen: Size) {
        self.screen = screen;
        self.update();
    }

    pub(crate) fn set_view_box(&mut self, view_box: Rect) {
        self.view_box = view_box;
        self.update();
    }

    pub(crate) fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.update();
    }

    pub(crate) fn move_to(&mut self, position: Point) {
        self.view_box.center = position;
        self.update();
    }

    pub(crate) fn convert_screen_to_world(&self, point: Point) -> Point {
        self.screen_to_world.transform(point)
    }

    pub(crate) fn clip_matrix(&self) -> Matrix4x4 {
        self.world_to_clip.to_matrix()
    }

    fn update(&mut self) {
        // fit the view_box to screen
        let is_horizontal = self.screen.width * self.view_box.size.height < self.screen.height * self.view_box.size.width;
        self.screen_to_world = self.calculate_screen_to_world(is_horizontal);
        self.world_to_clip = self.calculate_world_to_clip(is_horizontal);
        self.timestamp += 1;
    }

    fn calculate_screen_to_world(&self, is_horizontal: bool) -> OrthoNoRotTransformer {
        let s = if is_horizontal {
            self.view_box.size.width / self.screen.width
        } else {
            self.view_box.size.height / self.screen.height
        };

        let sx = s;
        let sy = -s;

        let tx = self.view_box.min_x();
        let ty = self.view_box.min_y() + s * self.screen.height;

        OrthoNoRotTransformer {
            sx,
            sy,
            tx,
            ty,
        }
    }

    fn calculate_world_to_clip(&self, is_horizontal: bool) -> OrthoNoRotTransformer {
        let (sx, sy) = if is_horizontal {
            let sx = 2.0 / self.view_box.size.width;
            let sy = 2.0 * self.screen.width / (self.view_box.size.width * self.screen.height);
            (sx, sy)
        } else {
            let sx = 2.0 * self.screen.height / (self.view_box.size.height * self.screen.width);
            let sy = 2.0 / self.view_box.size.height;
            (sx, sy)
        };

        let tx = -sx * self.view_box.center.x;
        let ty = -sy * self.view_box.center.y;

        OrthoNoRotTransformer {
            sx,
            sy,
            tx,
            ty,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eye::camera::OrthoNoRotCamera;
    use crate::geometry::point::Point;
    use crate::geometry::rect::Rect;
    use crate::geometry::size::Size;

    #[test]
    fn test_0() {
        let view_box = Rect::new(
            Point { x: 3.0, y: 1.0 },
            Point { x: 9.0, y: 5.0 },
        );

        let screen = Size {
            width: 3.0,
            height: 2.0,
        };

        let camera = OrthoNoRotCamera::new(screen, view_box);

        let world = camera.screen_to_world.transform(Point { x: 1.0, y: 0.5 });

        assert_points_eq(world, Point { x: 5.0, y: 4.0 }, 0.0001);
    }

    #[test]
    fn test_1() {
        let view_box = Rect::new(
            Point { x: 3.0, y: 1.0 },
            Point { x: 9.0, y: 5.0 },
        );

        let screen = Size {
            width: 3.0,
            height: 2.0,
        };

        let camera = OrthoNoRotCamera::new(screen, view_box);

        let clip = camera.world_to_clip.transform(Point { x: 5.0, y: 4.0 });

        assert_points_eq(clip, Point { x: -1.0 / 3.0, y: 0.5 }, 0.0001);
    }

    pub(crate) fn assert_points_eq(p1: Point, p2: Point, epsilon: f32) {
        assert!(
            (p1.x - p2.x).abs() < epsilon,
            "X coordinates differ: left = {}, right = {}",
            p1.x,
            p2.x
        );
        assert!(
            (p1.y - p2.y).abs() < epsilon,
            "Y coordinates differ: left = {}, right = {}",
            p1.y,
            p2.y
        );
    }
}