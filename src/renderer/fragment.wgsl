
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let result = vec3<f32>(in.uv, 0.0);
    //let result = pow(result, vec3<f32>(1.0/2.2));
    return vec4<f32>(result, 1.0);
}