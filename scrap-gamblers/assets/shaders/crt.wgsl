#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var screen_texture: texture_2d<f32>;
@group(2) @binding(1) var screen_sampler: sampler;
@group(2) @binding(2) var<uniform> params: CrtParams;

struct CrtParams {
    // x = distortion strength, y = scanline intensity, z = time, w = unused
    settings: vec4<f32>,
    // x = render target width, y = render target height, z = unused, w = unused
    resolution: vec4<f32>,
}

// Barrel / fish-eye distortion: bulges center outward
fn barrel(uv: vec2<f32>, k: f32) -> vec2<f32> {
    let c = uv * 2.0 - vec2<f32>(1.0);
    let r2 = dot(c, c);
    let d = c * (1.0 + k * r2 + k * 0.4 * r2 * r2);
    return d * 0.5 + vec2<f32>(0.5);
}

// Phosphor scanline darkening
fn scanline(uv_y: f32, intensity: f32) -> f32 {
    let s = sin(uv_y * params.resolution.y * 3.14159265);
    return 1.0 - intensity * (1.0 - s * s);
}

// Screen-edge vignette
fn vignette(uv: vec2<f32>) -> f32 {
    let v = uv * (1.0 - uv.yx);
    return clamp(pow(v.x * v.y * 18.0, 0.45), 0.0, 1.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let k = params.settings.x;
    let scan_int = params.settings.y;
    let t = params.settings.z;

    // Fish-eye barrel warp
    let duv = barrel(uv, k);

    // Hard black border outside warped bounds
    if duv.x < 0.0 || duv.x > 1.0 || duv.y < 0.0 || duv.y > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    // Soft edge mask so warped corners fade cleanly
    let edge = 0.004;
    let mask =
        smoothstep(0.0, edge, duv.x) *
        smoothstep(1.0, 1.0 - edge, duv.x) *
        smoothstep(0.0, edge, duv.y) *
        smoothstep(1.0, 1.0 - edge, duv.y);

    // Sample the menu render-target texture
    let raw = textureSample(screen_texture, screen_sampler, duv).rgb;

    // Phosphor green tint
    var color = raw * vec3<f32>(0.18, 1.0, 0.18);

    // Scanlines
    color *= scanline(duv.y, scan_int);

    // Subtle phosphor bleed / glow from neighbors
    let px = vec2<f32>(2.0 / params.resolution.x, 0.0);
    let py = vec2<f32>(0.0, 2.0 / params.resolution.y);
    let glow =
        textureSample(screen_texture, screen_sampler, duv + px).r +
        textureSample(screen_texture, screen_sampler, duv - px).r +
        textureSample(screen_texture, screen_sampler, duv + py).r +
        textureSample(screen_texture, screen_sampler, duv - py).r;
    color += vec3<f32>(0.015, 0.10, 0.015) * glow * 0.25;

    // Vignette (computed on original UV for symmetry)
    color *= vignette(uv) * 0.45 + 0.55;

    // Subtle per-frame brightness flicker
    let flicker = 0.978 + 0.022 * sin(t * 31.4 + uv.y * 2.7);
    color *= flicker;

    // Fade at warped edges
    color *= mask;

    return vec4<f32>(color, 1.0);
}
