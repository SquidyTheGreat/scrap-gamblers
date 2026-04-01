use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::Material2d,
};

/// GPU uniform block — two vec4s (32 bytes, trivially aligned).
#[derive(Clone, Debug, ShaderType)]
pub struct CrtParams {
    /// x = barrel-distortion k, y = scanline intensity, z = elapsed time, w = unused
    pub settings: Vec4,
    /// x = render-target width (px), y = height (px), z/w = unused
    pub resolution: Vec4,
}

impl Default for CrtParams {
    fn default() -> Self {
        Self {
            settings: Vec4::new(0.10, 0.30, 0.0, 0.0),
            resolution: Vec4::new(512.0, 320.0, 0.0, 0.0),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct CrtMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub params: CrtParams,
}

impl Material2d for CrtMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/crt.wgsl".into()
    }
}

/// Marker so we can look up the CRT quad to read its material handle.
#[derive(Component)]
pub struct CrtScreen;

/// Advances the `time` field of the CRT material each frame to drive flicker.
pub fn tick_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<CrtMaterial>>,
    query: Query<&MeshMaterial2d<CrtMaterial>, With<CrtScreen>>,
) {
    for handle in &query {
        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.params.settings.z = time.elapsed_secs();
        }
    }
}
