
struct VertexInput {
    @location(0) position: vec2f,
    @location(1) tex_coords: vec2f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
};

var<push_constant> viewport: mat4x4f;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = viewport * vec4<f32>(model.position, 0.0, 1.0);

    return out;
}

@group(0) @binding(0)
var t_tex: texture_2d<f32>;
@group(0) @binding(1)
var s_tex: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_tex, s_tex, in.tex_coords);
}