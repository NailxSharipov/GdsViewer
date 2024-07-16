use wgpu::{Device, Queue, TextureView};

pub(crate) struct DrawContext<'a> {
    pub(crate) device: &'a Device,
    pub(crate) queue: &'a Queue,
    pub(crate) view: TextureView
}