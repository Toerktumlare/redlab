struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(3) @binding(0)
var atlas: texture_2d<f32>;

@group(3) @binding(1)
var atlas_sampler: sampler;

@group(3) @binding(2)
var<uniform> uv_offset: vec2<f32>;

@group(3) @binding(3)
var<uniform> uv_scale: vec2<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * uv_scale + uv_offset;
    return textureSample(atlas, atlas_sampler, uv);
}

