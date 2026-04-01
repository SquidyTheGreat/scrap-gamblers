mod buttons;
mod crt_material;
mod menu;
mod pip_boy;

use bevy::prelude::*;
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};
use bevy::sprite_render::Material2dPlugin;
use crt_material::CrtMaterial;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pip-Boy 3000".into(),
                    resolution: (1024_u32, 640_u32).into(),
                    // Canvas selector used by trunk/wasm-bindgen
                    canvas: Some("#bevy".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(Material2dPlugin::<CrtMaterial>::default())
        .init_resource::<menu::MenuState>()
        .add_message::<menu::NavEvent>()
        .add_systems(
            Startup,
            (pip_boy::setup, buttons::setup).chain(),
        )
        .add_systems(
            Update,
            (
                buttons::handle_interaction,
                buttons::handle_keyboard,
                menu::on_nav_event,
                menu::update_text,
                crt_material::tick_time,
            ),
        )
        .run();
}
