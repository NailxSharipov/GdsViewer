use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Pencil {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Default for Pencil {
    fn default() -> Self {
        Pencil {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0
        }
    }
}