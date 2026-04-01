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

use crate::{
    crt_material::{CrtMaterial, CrtParams, CrtScreen},
    menu::{MenuItem, MenuState},
};

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
    menu: Res<MenuState>,
) {
    let rt = create_render_target(&mut images);
    spawn_cameras(&mut commands, rt.clone());
    spawn_world_geometry(&mut commands, &mut meshes, &mut crt_mats, rt);
    spawn_menu_text(&mut commands, &menu);
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

    // ── Panel decorations ─────────────────────────────────────────────────
    left_panel(commands, &wl);
    right_panel(commands, &wl);
}

fn left_panel(commands: &mut Commands, wl: &RenderLayers) {
    let cx = -390.0_f32;

    // Panel background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.100, 0.130, 0.072),
            custom_size: Some(Vec2::new(130.0, 380.0)),
            ..default()
        },
        Transform::from_xyz(cx, 0.0, -6.0),
        wl.clone(),
    ));

    // Power LED
    commands.spawn((
        Sprite {
            color: Color::srgb(0.10, 0.90, 0.20),
            custom_size: Some(Vec2::splat(10.0)),
            ..default()
        },
        Transform::from_xyz(cx, 200.0, -5.0),
        wl.clone(),
    ));

    // Decorative horizontal slots
    for i in 0..3_i32 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.070, 0.090, 0.052),
                custom_size: Some(Vec2::new(90.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(cx, 120.0 - i as f32 * 60.0, -5.0),
            wl.clone(),
        ));
    }

    // Speaker grille dot grid
    for row in 0..5_i32 {
        for col in 0..4_i32 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.06, 0.08, 0.04),
                    custom_size: Some(Vec2::splat(5.0)),
                    ..default()
                },
                Transform::from_xyz(
                    cx - 18.0 + col as f32 * 12.0,
                    -100.0 - row as f32 * 12.0,
                    -5.0,
                ),
                wl.clone(),
            ));
        }
    }
}

fn right_panel(commands: &mut Commands, wl: &RenderLayers) {
    let cx = 390.0_f32;

    // Panel background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.100, 0.130, 0.072),
            custom_size: Some(Vec2::new(130.0, 380.0)),
            ..default()
        },
        Transform::from_xyz(cx, 0.0, -6.0),
        wl.clone(),
    ));

    // Two rotary knob widgets
    for i in 0..2_i32 {
        let y = 160.0 - i as f32 * 100.0;
        // Ring
        commands.spawn((
            Sprite {
                color: Color::srgb(0.09, 0.11, 0.06),
                custom_size: Some(Vec2::splat(48.0)),
                ..default()
            },
            Transform::from_xyz(cx, y, -5.5),
            wl.clone(),
        ));
        // Centre dot
        commands.spawn((
            Sprite {
                color: Color::srgb(0.05, 0.07, 0.03),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            Transform::from_xyz(cx, y, -5.0),
            wl.clone(),
        ));
        // Tick mark
        commands.spawn((
            Sprite {
                color: Color::srgb(0.28, 0.38, 0.18),
                custom_size: Some(Vec2::new(3.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(cx, y + 8.0, -4.9),
            wl.clone(),
        ));
    }

    // Decorative slots
    for i in 0..3_i32 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.070, 0.090, 0.052),
                custom_size: Some(Vec2::new(90.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(cx, -60.0 - i as f32 * 55.0, -5.0),
            wl.clone(),
        ));
    }
}

// ── Menu text (rendered into the CRT render-target) ───────────────────────────

fn spawn_menu_text(commands: &mut Commands, menu: &MenuState) {
    let ml = RenderLayers::layer(MENU_LAYER);

    // Title
    commands.spawn((
        Text2d::new("═══  PIP-BOY  3000  ═══"),
        TextFont { font_size: 21.0, ..default() },
        TextColor(Color::srgb(0.90, 1.0, 0.90)),
        Transform::from_xyz(0.0, 128.0, 0.0),
        ml.clone(),
    ));

    // Divider
    commands.spawn((
        Text2d::new("────────────────────────────"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.50)),
        Transform::from_xyz(0.0, 104.0, 0.0),
        ml.clone(),
    ));

    // Menu items
    let item_spacing = 33.0_f32;
    let items_top = 68.0_f32;
    for (i, &label) in menu.items.iter().enumerate() {
        let y = items_top - i as f32 * item_spacing;
        let selected = i == menu.selected;
        commands.spawn((
            Text2d::new(format!("{} {}", if selected { ">" } else { " " }, label)),
            TextFont { font_size: 19.0, ..default() },
            TextColor(if selected {
                Color::WHITE
            } else {
                Color::srgba(0.55, 0.80, 0.55, 0.85)
            }),
            Transform::from_xyz(-40.0, y, 0.0),
            ml.clone(),
            MenuItem { index: i, label },
        ));
    }

    // Footer hint
    commands.spawn((
        Text2d::new("[↑↓] NAV    [ENTER] SELECT"),
        TextFont { font_size: 11.5, ..default() },
        TextColor(Color::srgba(0.40, 0.65, 0.40, 0.65)),
        Transform::from_xyz(0.0, -132.0, 0.0),
        ml.clone(),
    ));
}
