use std::mem;
use wgpu::{Buffer, BufferSlice, BufferUsages, Device};
use wgpu::util::DeviceExt;
use crate::eye::camera::OrthoNoRotCamera;

pub(crate) struct GeometryCommonBuffers {
    pub(crate) vertex: Buffer,
    pub(crate) index: Buffer,
    pub(crate) transform: Buffer,
}

impl GeometryCommonBuffers {
    pub(crate) fn new(camera: &OrthoNoRotCamera, device: &Device, vertex_capacity: usize, index_capacity: usize) -> Self {
        let vertex = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: Self::vertex_size(vertex_capacity) as wgpu::BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: Self::index_size(index_capacity) as wgpu::BufferAddress,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&camera.clip_matrix()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        Self {
            vertex,
            index,
            transform,
        }
    }

    #[inline]
    pub(crate) fn vertex_size(count: usize) -> usize {
        count * mem::size_of::<[f32; 2]>()
    }

    #[inline]
    pub(crate) fn index_size(count: usize) -> usize {
        count * mem::size_of::<u32>()
    }

    #[inline]
    pub(crate) fn vertex_slice(&self, count: usize) -> BufferSlice<'_> {
        self.vertex.slice(0..Self::vertex_size(count) as u64)
    }

    #[inline]
    pub(crate) fn index_slice(&self, count: usize) -> BufferSlice<'_> {
        self.index.slice(0..Self::index_size(count) as u64)
    }
}