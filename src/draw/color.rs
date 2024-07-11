#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

pub(crate) const COLORS: [Color; 16] = [
    Color { r: 0.94, g: 0.31, b: 0.31 }, // Red
    Color { r: 0.31, g: 0.94, b: 0.31 }, // Green
    Color { r: 0.31, g: 0.31, b: 0.94 }, // Blue
    Color { r: 0.94, g: 0.94, b: 0.31 }, // Yellow
    Color { r: 0.94, g: 0.63, b: 0.31 }, // Orange
    Color { r: 0.63, g: 0.31, b: 0.94 }, // Purple
    Color { r: 0.31, g: 0.94, b: 0.94 }, // Cyan
    Color { r: 0.94, g: 0.31, b: 0.94 }, // Magenta
    Color { r: 0.63, g: 0.63, b: 0.63 }, // Gray
    Color { r: 0.94, g: 0.75, b: 0.31 }, // Gold
    Color { r: 0.75, g: 0.31, b: 0.94 }, // Violet
    Color { r: 0.31, g: 0.94, b: 0.63 }, // Spring Green
    Color { r: 0.94, g: 0.31, b: 0.63 }, // Pink
    Color { r: 0.63, g: 0.94, b: 0.31 }, // Lime Green
    Color { r: 0.94, g: 0.94, b: 0.94 }, // White
    Color { r: 0.31, g: 0.31, b: 0.31 }, // Black
];