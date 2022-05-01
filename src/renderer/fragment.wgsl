
struct Input {
    [[location(0)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fs_main(in: Input) -> [[location(0)]] vec4<f32> {
    let result = vec3<f32>(in.uv, 0.0);
    return vec4<f32>(result, 1.0);
}