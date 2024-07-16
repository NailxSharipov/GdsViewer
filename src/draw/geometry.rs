use wgpu::{BufferUsages, ColorTargetState, Device, util::DeviceExt, BufferAddress};
use crate::control::navigation::NavigationEvent;
use crate::draw::buffers::GeometryCommonBuffers;
use crate::draw::context::DrawContext;
use crate::draw::document::Document;
use crate::draw::fill::render::FillRender;
use crate::draw::painter::Painter;
use crate::draw::stroke::render::StrokeRender;
use crate::draw::triangulation::PolygonSize;
use crate::eye::camera::OrthoNoRotCamera;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;

pub(crate) struct GeometryPainter {
    pub(crate) document: Document,
    camera: OrthoNoRotCamera,
    fill_render: FillRender,
    stroke_render: StrokeRender,
    common_buffers: GeometryCommonBuffers,
    camera_timestamp: usize,
    start_zoom: f32,
    start_dragged: Point,
    stroke_width: f32,
}

impl GeometryPainter {
    pub(crate) fn create(color: ColorTargetState, device: &Device, screen_width: u32, screen_height: u32) -> Self {
        let document = Document::polygons();

        let camera = OrthoNoRotCamera::new(
            Size::new_uint(screen_width, screen_height),
            Rect::with_size(document.size),
        );

        let mut vertex_capacity = 0;
        let mut index_capacity = 0;
        for layer in document.layers.iter() {
            let n = layer.shapes.vertices_count();
            vertex_capacity = vertex_capacity.max(layer.fill_mesh.points.len());
            vertex_capacity = vertex_capacity.max(4 * n);

            index_capacity = index_capacity.max(layer.fill_mesh.indices.len());
            index_capacity = index_capacity.max(6 * n)
        }

        let common_buffers = GeometryCommonBuffers::new(&camera, device, vertex_capacity, index_capacity);
        let fill_render = FillRender::new(&color, &common_buffers, device);
        let stroke_render = StrokeRender::new(&color, &common_buffers, device);

        Self {
            document,
            common_buffers,
            fill_render,
            stroke_render,
            camera,
            camera_timestamp: usize::MAX,
            start_zoom: 1.0,
            start_dragged: Point { x: 0.0, y: 0.0 },
            stroke_width: 2.0,
        }
    }

    fn update_transform_buffer(&mut self, context: &mut DrawContext) {
        if self.camera.timestamp() == self.camera_timestamp {
            return;
        }
        self.camera_timestamp = self.camera.timestamp();

        let ortho_matrix = self.camera.clip_matrix();

        let staging_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: bytemuck::cast_slice(&ortho_matrix),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Update Transform Buffer Encoder"),
        });

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.common_buffers.transform,
            0,
            std::mem::size_of::<[f32; 16]>() as BufferAddress,
        );

        context.queue.submit(Some(encoder.finish()));
    }
}

impl Painter for GeometryPainter {
    fn draw(&mut self, context: &mut DrawContext) {
        self.update_transform_buffer(context);
        let width = self.camera.convert_size_screen_to_world(self.stroke_width);

        let mut clear = true;
        for layer in self.document.layers.iter_mut() {
            layer.build_strokes(width);
            self.fill_render.draw(clear, &layer.fill_mesh, layer.brush, &self.common_buffers, context);
            self.stroke_render.draw(&layer.stroke_mesh, layer.pencil, &self.common_buffers, context);
            clear = false;
        }
    }

    fn update_size(&mut self, size: Size) {
        self.camera.set_screen(size);
    }

    fn navigation_event(&mut self, navigation_event: NavigationEvent) {
        match navigation_event {
            NavigationEvent::StartZoom(s) => {
                self.start_zoom = self.camera.zoom();
                self.camera.set_zoom(self.start_zoom * s.scale, s.cursor);
            }
            NavigationEvent::ProcessZoom(e) => {
                self.camera.set_zoom(self.start_zoom * e.scale, e.cursor);
            }
            NavigationEvent::EndZoom(e) => {
                self.camera.set_zoom(self.start_zoom * e.scale, e.cursor);
            }
            NavigationEvent::CancelZoom(e) => {
                self.camera.set_zoom(self.start_zoom * e.scale, e.cursor);
            }
            NavigationEvent::StartDragged(_e) => {
                self.start_dragged = self.camera.world_position();
            }
            NavigationEvent::EndDragged(_e) => {}
            NavigationEvent::MoveDragged(e) => {
                let screen_delta = e.start - e.current;
                let world_delta = self.camera.convert_vector_screen_to_world(screen_delta);
                self.camera.move_to(self.start_dragged + world_delta);
            }
        }
    }
}