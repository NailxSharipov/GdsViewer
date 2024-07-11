use std::borrow::Cow;
use wgpu::{Buffer, BufferUsages, ColorTargetState, Device, Queue, RenderPipeline, TextureView, util::DeviceExt, BindGroup, BufferBindingType, ShaderStages, BufferAddress, BindGroupLayout};
use crate::control::navigation::NavigationEvent;
use crate::draw::brush::Brush;
use crate::draw::document::Document;
use crate::draw::painter::Painter;
use crate::eye::camera::OrthoNoRotCamera;
use crate::geometry::point::Point;
use crate::geometry::rect::Rect;
use crate::geometry::size::Size;

pub(crate) struct GeometryPainter {
    pub(crate) document: Document,
    camera: OrthoNoRotCamera,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    brush_buffer: Buffer,
    index_buffer: Buffer,
    transform_buffer: Buffer,
    bind_group: BindGroup,
    camera_timestamp: usize,
    start_zoom: f32
}

impl GeometryPainter {
    pub(crate) fn create(color: ColorTargetState, device: &Device, screen_width: u32, screen_height: u32) -> Self {
        let doc_size = Size::new(1000.0, 1000.0);

        let document = Document::five(doc_size);

        let camera = OrthoNoRotCamera::new(
            Size::new_uint(screen_width, screen_height),
            Rect::with_size(doc_size),
        );

        // Create GPU buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&document.mesh.points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let brush_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Brush Buffer"),
            contents: bytemuck::cast_slice(&document.mesh.brushes),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&document.mesh.indices),
            usage: BufferUsages::INDEX,
        });

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&camera.clip_matrix()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let brush_data: Vec<[f32; 8]> = Brush::create_set().iter().map(|brush| {
            [
                brush.vec.x,
                brush.vec.y,
                brush.width,
                brush.color.r,
                brush.color.g,
                brush.color.b,
                0.0,
                0.0,
            ]
        }).collect();

        let brush_set_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Brush Data Buffer"),
            contents: bytemuck::cast_slice(&brush_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(512), // Updated size
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: brush_set_buffer.as_entire_binding(),
                },
            ],
        });

        let render_pipeline = Self::build_pipeline(color, device, bind_group_layout);

        Self {
            document,
            render_pipeline,
            vertex_buffer,
            brush_buffer,
            index_buffer,
            transform_buffer,
            bind_group,
            camera,
            camera_timestamp: usize::MAX,
            start_zoom: 1.0,
        }
    }

    fn build_pipeline(color: ColorTargetState, device: &Device, bind_group_layout: BindGroupLayout) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let color_target_state = ColorTargetState {
            format: color.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        };

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<u32>() as BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Uint32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(color_target_state)],

            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Front),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    fn update_vertex_buffer(&self, device: &Device, queue: &Queue) {
        let mesh = &self.document.mesh;

        // Create a staging buffer with the updated vertex data
        let staging_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: bytemuck::cast_slice(&mesh.points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        // Create a command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Update Vertex Buffer Encoder"),
        });

        // Copy data from the staging buffer to the vertex buffer
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.vertex_buffer,
            0,
            (mesh.points.len() * std::mem::size_of::<[f32; 2]>()) as BufferAddress,
        );

        // Submit the command buffer
        queue.submit(Some(encoder.finish()));
    }

    fn update_transform_buffer(&mut self, device: &Device, queue: &Queue) {
        if self.camera.timestamp() == self.camera_timestamp {
            return;
        }
        self.camera_timestamp = self.camera.timestamp();

        let ortho_matrix = self.camera.clip_matrix();

        let staging_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: bytemuck::cast_slice(&ortho_matrix),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Update Transform Buffer Encoder"),
        });

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.transform_buffer,
            0,
            std::mem::size_of::<[f32; 16]>() as BufferAddress,
        );

        queue.submit(Some(encoder.finish()));
    }
}

impl Painter for GeometryPainter {
    fn draw(&mut self, queue: &Queue, device: &Device, view: &TextureView) {
        self.update_vertex_buffer(device, queue);
        self.update_transform_buffer(device, queue);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.brush_buffer.slice(..));
            rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw_indexed(0..self.document.mesh.indices.len() as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }

    fn update_size(&mut self, size: Size) {
        self.camera.set_screen(size);
    }

    fn update_pos(&mut self, pos: Point) {
        self.camera.move_to(pos);
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
        }
    }
}