use std::mem;
use wgpu::{BindGroup, Buffer, BufferAddress, BufferBindingType, BufferUsages, ColorTargetState, Face, RenderPipeline, ShaderStages};
use wgpu::Device;
use wgpu::util::DeviceExt;
use crate::draw::buffers::GeometryCommonBuffers;
use crate::draw::context::DrawContext;
use crate::draw::index_mesh::ListMesh;
use crate::draw::stroke::pencil::Pencil;

pub(crate) struct StrokeRender {
    pencil_buffer: Buffer,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
}

impl StrokeRender {
    pub(crate) fn new(color: &ColorTargetState, common_buffers: &GeometryCommonBuffers, device: &Device) -> Self {
        let pencil_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Pencil Buffer"),
            size: mem::size_of::<Pencil>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST | BufferUsages::VERTEX,
            mapped_at_creation: false,
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
                        min_binding_size: None,
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
                    resource: common_buffers.transform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: pencil_buffer.as_entire_binding(),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<[f32; 2]>() as BufferAddress,
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
                        array_stride: mem::size_of::<u32>() as BufferAddress,
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
                targets: &[Some(ColorTargetState {
                    format: color_target_state.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { pencil_buffer, bind_group, pipeline }
    }

    pub(crate) fn draw(&self, mesh: &ListMesh, pencil: Pencil, buffers: &GeometryCommonBuffers, context: &DrawContext) {

        // update_buffers

        let vertex_staging_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Staging Buffer"),
            contents: bytemuck::cast_slice(&mesh.points),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        let index_staging_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Staging Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        let staging_brush_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Brush Buffer"),
            contents: bytemuck::cast_slice(&[pencil]),
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        });

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Update Stroke Buffers Encoder"),
        });

        encoder.copy_buffer_to_buffer(
            &vertex_staging_buffer,
            0,
            &buffers.vertex,
            0,
            (mesh.points.len() * mem::size_of::<[f32; 2]>()) as BufferAddress,
        );

        encoder.copy_buffer_to_buffer(
            &index_staging_buffer,
            0,
            &buffers.index,
            0,
            (mesh.indices.len() * mem::size_of::<u32>()) as BufferAddress,
        );

        encoder.copy_buffer_to_buffer(
            &staging_brush_buffer,
            0,
            &self.pencil_buffer,
            0,
            mem::size_of::<Pencil>() as BufferAddress,
        );

        context.queue.submit(Some(encoder.finish()));

        // set pipeline and draw

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &context.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, buffers.vertex_slice(mesh.points.len()));
            rpass.set_vertex_buffer(1, self.pencil_buffer.slice(..));
            rpass.set_index_buffer(buffers.index_slice(mesh.indices.len()), wgpu::IndexFormat::Uint32);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
        }

        context.queue.submit(Some(encoder.finish()));
    }
}