use bevy::{
    prelude::*,
    camera::{
        ScalingMode,
        RenderTarget,
        ImageRenderTarget,
        visibility::RenderLayers,
    },
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

use crate::crt_material::{CrtMaterial, CrtParams, CrtScreen};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Render-target texture resolution — must match CrtParams::resolution.
const RT_W: u32 = 512;
const RT_H: u32 = 320;

const SCREEN_W: f32 = RT_W as f32;
const SCREEN_H: f32 = RT_H as f32;

/// Slight upward offset of the screen within the Pip-Boy body.
const SCREEN_Y: f32 = 18.0;

const MENU_LAYER: usize = 1;
const WORLD_LAYER: usize = 0;

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut crt_mats: ResMut<Assets<CrtMaterial>>,
) {
    let rt = create_render_target(&mut images);
    spawn_cameras(&mut commands, rt.clone());
    spawn_world_geometry(&mut commands, &mut meshes, &mut crt_mats, rt);
}

// ── Render target ─────────────────────────────────────────────────────────────

fn create_render_target(images: &mut Assets<Image>) -> Handle<Image> {
    let size = Extent3d {
        width: RT_W,
        height: RT_H,
        depth_or_array_layers: 1,
    };
    let mut img = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("crt_render_target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    img.resize(size);
    images.add(img)
}

// ── Cameras ───────────────────────────────────────────────────────────────────

fn spawn_cameras(commands: &mut Commands, rt: Handle<Image>) {
    // Menu camera: renders layer-1 text entities into the CRT texture.
    // RenderTarget is a separate Component in Bevy 0.18 (required by Camera).
    commands.spawn((
        Camera2d,
        // Override the default orthographic projection to match the render target size.
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: RT_W as f32,
                height: RT_H as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.012, 0.045, 0.012)),
            ..default()
        },
        RenderTarget::Image(ImageRenderTarget {
            handle: rt,
            scale_factor: 1.0,
        }),
        RenderLayers::layer(MENU_LAYER),
    ));

    // Main camera: renders Pip-Boy frame, CRT quad, and UI buttons.
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        RenderLayers::layer(WORLD_LAYER),
    ));
}

// ── World-space geometry ──────────────────────────────────────────────────────

fn spawn_world_geometry(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    crt_mats: &mut Assets<CrtMaterial>,
    rt: Handle<Image>,
) {
    let wl = RenderLayers::layer(WORLD_LAYER);

    // Dark environment fill
    commands.spawn((
        Sprite {
            color: Color::srgb(0.030, 0.045, 0.020),
            custom_size: Some(Vec2::splat(4000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -20.0),
        wl.clone(),
    ));

    // ── Pip-Boy body ──────────────────────────────────────────────────────

    commands.spawn((
        Sprite {
            color: Color::srgb(0.140, 0.175, 0.100),
            custom_size: Some(Vec2::new(900.0, 540.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -8.0),
        wl.clone(),
    ));

    // Top highlight strip
    commands.spawn((
        Sprite {
            color: Color::srgb(0.200, 0.250, 0.140),
            custom_size: Some(Vec2::new(900.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 270.0, -7.5),
        wl.clone(),
    ));

    // Bottom shadow strip
    commands.spawn((
        Sprite {
            color: Color::srgb(0.070, 0.090, 0.050),
            custom_size: Some(Vec2::new(900.0, 8.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -270.0, -7.5),
        wl.clone(),
    ));

    // ── Screen bezel ──────────────────────────────────────────────────────

    // Outer bezel (dark frame)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.055, 0.065, 0.040),
            custom_size: Some(Vec2::new(SCREEN_W + 38.0, SCREEN_H + 38.0)),
            ..default()
        },
        Transform::from_xyz(0.0, SCREEN_Y, -2.0),
        wl.clone(),
    ));

    // Inner glow rim (phosphor bleed at bezel edge)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.040, 0.150, 0.055),
            custom_size: Some(Vec2::new(SCREEN_W + 14.0, SCREEN_H + 14.0)),
            ..default()
        },
        Transform::from_xyz(0.0, SCREEN_Y, -1.0),
        wl.clone(),
    ));

    // ── CRT screen quad ───────────────────────────────────────────────────

    let mat = crt_mats.add(CrtMaterial {
        texture: rt,
        params: CrtParams::default(),
    });
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(SCREEN_W, SCREEN_H))),
        MeshMaterial2d(mat),
        Transform::from_xyz(0.0, SCREEN_Y, 0.0),
        wl.clone(),
        CrtScreen,
    ));

}

