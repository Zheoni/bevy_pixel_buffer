// From bevy game of life compute shader example

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

fn get_cell(location: vec2<i32>, offset_x: i32, offset_y: i32) -> i32 {
    let value: vec4<f32> = textureLoad(texture, location + vec2<i32>(offset_x, offset_y));
    return i32(value.x);
}

fn count_alive(location: vec2<i32>) -> i32 {
    return get_cell(location, -1, -1) +
           get_cell(location, -1,  0) +
           get_cell(location, -1,  1) +
           get_cell(location,  0, -1) +
           get_cell(location,  0,  1) +
           get_cell(location,  1, -1) +
           get_cell(location,  1,  0) +
           get_cell(location,  1,  1);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let n_alive = count_alive(location);
    let color = vec4<f32>(f32(n_alive) / 8.0);

    var alive: bool;
    if (n_alive == 3) {
        alive = true;
    } else if (n_alive == 2) {
        let currently_alive = get_cell(location, 0, 0);
        alive = bool(currently_alive);
    } else {
        alive = false;
    }

    storageBarrier();

    textureStore(texture, location, vec4<f32>(f32(alive)));
}