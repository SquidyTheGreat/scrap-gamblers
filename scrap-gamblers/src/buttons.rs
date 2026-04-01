use bevy::prelude::*;
use crate::menu::NavEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum NavAction {
    Up,
    Down,
    Select,
}

#[derive(Component, Clone)]
pub struct PipBoyButton {
    pub action: NavAction,
}

// ── Colours ──────────────────────────────────────────────────────────────────

const BG_NORMAL: Color = Color::srgb(0.14, 0.18, 0.10);
const BG_HOVER:  Color = Color::srgb(0.20, 0.26, 0.14);
const BG_PRESS:  Color = Color::srgb(0.09, 0.11, 0.06);

const BORDER_NORMAL: Color = Color::srgb(0.32, 0.42, 0.22);
const BORDER_PRESS:  Color = Color::srgb(0.18, 0.24, 0.12);

const TEXT_COLOR: Color = Color::srgb(0.55, 0.85, 0.35);

// ── Setup ─────────────────────────────────────────────────────────────────────

pub fn setup(mut commands: Commands) {
    // Transparent full-screen flex row — buttons float over the 2-D world.
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::horizontal(Val::Px(52.0)),
            ..default()
        })
        .with_children(|root| {
            // ── Left panel: UP + DOWN ─────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|panel| {
                panel_label(panel, "NAVIGATE");
                spawn_button(panel, NavAction::Up,   "▲  UP");
                spawn_button(panel, NavAction::Down, "▼  DOWN");
            });

            // ── Centre spacer: CRT mesh renders through it ────────────────
            root.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // ── Right panel: SELECT ───────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            })
            .with_children(|panel| {
                panel_label(panel, "ACTION");
                spawn_button(panel, NavAction::Select, "●  OK");
            });
        });
}

fn panel_label(parent: &mut ChildSpawnerCommands, text: &'static str) {
    parent.spawn((
        Text::new(text),
        TextFont { font_size: 11.0, ..default() },
        TextColor(Color::srgba(0.40, 0.60, 0.28, 0.70)),
    ));
}

fn spawn_button(parent: &mut ChildSpawnerCommands, action: NavAction, label: &'static str) {
    parent
        .spawn((
            Button,
            PipBoyButton { action },
            Node {
                width: Val::Px(108.0),
                height: Val::Px(48.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                // BorderRadius lives inside Node, not as a separate component
                border_radius: BorderRadius::all(Val::Px(7.0)),
                ..default()
            },
            BackgroundColor(BG_NORMAL),
            BorderColor::all(BORDER_NORMAL),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont { font_size: 15.0, ..default() },
                TextColor(TEXT_COLOR),
            ));
        });
}

// ── Systems ───────────────────────────────────────────────────────────────────

/// Handles mouse/pointer interaction on UI buttons.
pub fn handle_interaction(
    mut query: Query<
        (
            &Interaction,
            &PipBoyButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    mut events: MessageWriter<NavEvent>,
) {
    for (interaction, btn, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Pressed => {
                bg.0 = BG_PRESS;
                *border = BorderColor::all(BORDER_PRESS);
                events.write(match btn.action {
                    NavAction::Up     => NavEvent::Up,
                    NavAction::Down   => NavEvent::Down,
                    NavAction::Select => NavEvent::Select,
                });
            }
            Interaction::Hovered => {
                bg.0 = BG_HOVER;
                *border = BorderColor::all(BORDER_NORMAL);
            }
            Interaction::None => {
                bg.0 = BG_NORMAL;
                *border = BorderColor::all(BORDER_NORMAL);
            }
        }
    }
}

/// Keyboard shortcuts: arrows / W-S / Enter-Space.
pub fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<NavEvent>,
) {
    if keys.just_pressed(KeyCode::ArrowUp)   || keys.just_pressed(KeyCode::KeyW) {
        events.write(NavEvent::Up);
    }
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        events.write(NavEvent::Down);
    }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        events.write(NavEvent::Select);
    }
}
