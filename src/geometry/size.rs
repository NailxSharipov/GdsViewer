#[derive(Clone, Copy, Debug)]
pub(crate) struct Size {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl Size {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub(crate) fn new_uint(width: u32, height: u32) -> Self {
        Self { width: width as f32, height: height as f32 }
    }

    pub(crate) fn scaled(&self, scale: f32) -> Self {
        let width = scale * self.width;
        let height = scale * self.height;

        Size { width, height }
    }
}