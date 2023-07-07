@group(0) @binding(0)
var<uniform> vp_matrix: mat4x4<f32>;

@group(1) @binding(2)
var<uniform> grid_size: f32;

struct InstanceInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    var position: vec3<f32>;
    var texcoord: vec2<f32>;

    if in_vertex_index == u32(0) {
        position = vec3<f32>(0.0, 0.0, 0.0);
        texcoord = vec2<f32>(0.0, 0.0);
    } else if in_vertex_index == u32(1) {
        position = vec3<f32>(1.0, 0.0, 0.0);
        texcoord = vec2<f32>(1.0, 0.0);
    } else if in_vertex_index == u32(2) {
        position = vec3<f32>(1.0, 1.0, 0.0);
        texcoord = vec2<f32>(1.0, 1.0);
    } else if in_vertex_index == u32(3) {
        position = vec3<f32>(1.0, 1.0, 0.0);
        texcoord = vec2<f32>(1.0, 1.0);
    } else if in_vertex_index == u32(4) {
        position = vec3<f32>(0.0, 1.0, 0.0);
        texcoord = vec2<f32>(0.0, 1.0);
    } else if in_vertex_index == u32(5) {
        position = vec3<f32>(0.0, 0.0, 0.0);
        texcoord = vec2<f32>(0.0, 0.0);
    }

    position = position + instance.position;
    texcoord = (texcoord + instance.texcoord) / grid_size;

    var out: VertexOutput;
    out.clip_position = vp_matrix * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.texcoord);
}