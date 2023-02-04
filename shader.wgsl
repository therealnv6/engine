struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct Camera {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

struct Material {
    color: vec4<f32>,
};  

@group(0) @binding(0)
var<uniform> material: Material;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return material.color;
}