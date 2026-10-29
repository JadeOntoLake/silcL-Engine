// 着色器语言

struct VertexInput {
    @location(0) position: vec3<f32>,
};

// 相机移动矩阵 
struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};


// 顶点着色器
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.view_proj * vec4<f32>(in.position, 1.0);
    return out;
}

// 片段（面）着色器
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.4, 0.7, 0.9, 0.6);
}

// 片段（边）着色器
@fragment
fn ed_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.7, 0.5, 0.0, 0.7);
}

// 片段（x轴）着色器
@fragment
fn x_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 0.5);
}

// 片段（y轴·）着色器
@fragment
fn y_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 0.5);
}

// 片段（z轴）着色器
@fragment
fn z_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 1.0, 0.5);
}
