struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) frag_position: vec2<f32>,
};

struct Brush {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> brush: Brush;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = transform * vec4<f32>(in.position, 0.0, 1.0);
    out.frag_position = in.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(brush.r, brush.g, brush.b, brush.a);
}
