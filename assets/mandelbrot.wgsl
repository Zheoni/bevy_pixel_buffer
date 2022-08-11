@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct ParamsUniforms {
    max_iter: i32,
    scale: f32,
    center: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> params: ParamsUniforms;

fn square_complex(c: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(c.x * c.x - c.y * c.y, 2.0 * c.x * c.y);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let max_iter = params.max_iter;
    let center = params.center;
    let scale = params.scale;

    let pos = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let dimensions = textureDimensions(texture);
    let w = f32(dimensions.x);
    let h = f32(dimensions.y);
    let aspect = w / h;

    let x = scale * aspect * (f32(pos.x) - 0.5 * w) / w + center.x;
    let y = -scale * (f32(pos.y) - 0.5 * h) / h + center.y;

    var c: vec2<f32> = vec2<f32>(x, y);
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);

    var b: i32 = -1;
    for (var i: i32 = 0; i < max_iter; i++) {
        if ((z.x * z.x + z.y * z.y) > 4.0) {
            b = i;
            break;
        }
        z = square_complex(z) + c;
    }
    if (b == -1) {
        b = max_iter;
    }
    var intensity: f32 = f32(b) / f32(max_iter);
    intensity = (2.0 * intensity) / (abs(intensity) + 1.0);
    let r = max(0.0, 2.0 * intensity - 1.0);

    let color = vec4<f32>(r, intensity, intensity, 1.0);

    textureStore(texture, pos, color);
}