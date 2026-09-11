struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) p: vec2f,
};

struct PainterUniform {
    c: vec2f, // world space
    size: vec2f, // rect size in screen space

    translation: vec2f, // world space
    scale: f32, // world space
    max_iters: u32,

    _pad: vec3f,
    is_julia: u32, 
}

@group(0) @binding(0) var<uniform> data: PainterUniform;

// Special thanks to https://stackoverflow.com/a/16505538 for the color palette
const PALETTE_SIZE: u32 = 16;
const PALETTE: array<vec3f, PALETTE_SIZE> = array<vec3f, PALETTE_SIZE>(
    vec3f(66, 30, 15) / 255.0,
    vec3f(25, 7, 26) / 255.0,
    vec3f(9, 1, 47) / 255.0,
    vec3f(4, 4, 73) / 255.0,
    vec3f(0, 7, 100) / 255.0,
    vec3f(12, 44, 138) / 255.0,
    vec3f(24, 82, 177) / 255.0,
    vec3f(57, 125, 209) / 255.0,
    vec3f(134, 181, 229) / 255.0,
    vec3f(211, 236, 248) / 255.0,
    vec3f(241, 233, 191) / 255.0,
    vec3f(248, 201, 95) / 255.0,
    vec3f(255, 170, 0) / 255.0,
    vec3f(204, 128, 0) / 255.0,
    vec3f(153, 87, 0) / 255.0,
    vec3f(106, 52, 3) / 255.0,
);

fn pick_color(i: u32) -> vec3f {
    return select(vec3f(0), PALETTE[i % PALETTE_SIZE], i < data.max_iters && i > 0);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );

    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, -1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, 1.0)
    );

    var out: VertexOutput;
    out.position = vec4<f32>(pos[vertex_index], 0.0, 1.0);

    out.p = uvs[vertex_index] * data.size - data.size / 2.0;
    out.p = out.p / data.scale + data.translation;

    return out;
}

struct FragmentInput {
    @location(0) p: vec2f,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    var c: vec2f;
    var z: vec2f;
    if data.is_julia == 1{
        c = data.c;
        z = in.p;
    } else {
        c = in.p;
        z = vec2f(0.0);
    }

    var i: u32;
    for (i = 0u; i < data.max_iters; i++) {
        // z_(n+1) = (z_n)^2 + c
        z = vec2f(
            z.x * z.x - z.y * z.y + c.x,
            z.y * z.x + z.x * z.y + c.y
        );

        // squared magnitude
        let z_mag_sq = dot(z, z);
        if (z_mag_sq > 64.0) {
            break;
        }
    }

    return vec4(pick_color(i), 1.0);
}