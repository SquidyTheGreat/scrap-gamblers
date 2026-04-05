use bevy::{
    prelude::*,
    camera::visibility::RenderLayers,
    sprite::Anchor,
};

// Layer that the CRT menu camera renders.
const CRT_LAYER: usize = 1;
// Render-target width (must match pip_boy::RT_W).
const SCREEN_W: f32 = 512.0;

// ── Views ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Default)]
pub enum View {
    #[default]
    MainMenu,
    Status,
    Team,
    Map,
    Account,
}

#[derive(Resource, Default)]
pub struct CurrentView(pub View);

// ── Menu items ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum MenuAction {
    GoTo(View),
}

pub struct MenuItemDef {
    pub label: &'static str,
    pub action: MenuAction,
}

#[derive(Resource)]
pub struct MenuState {
    pub items: Vec<MenuItemDef>,
    pub selected: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            items: vec![
                MenuItemDef { label: "STATUS",  action: MenuAction::GoTo(View::Status)  },
                MenuItemDef { label: "TEAM",    action: MenuAction::GoTo(View::Team)    },
                MenuItemDef { label: "MAP",     action: MenuAction::GoTo(View::Map)     },
                MenuItemDef { label: "ACCOUNT", action: MenuAction::GoTo(View::Account) },
            ],
            selected: 0,
        }
    }
}

// ── Navigation events ─────────────────────────────────────────────────────────

#[derive(Message, Clone, Copy)]
pub enum NavEvent {
    Up,
    Down,
    Select,
    Back,
}

/// Marks a text entity that represents a menu row (used by `update_text`).
#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
    pub label: &'static str,
}

/// Marks every entity that belongs to the currently active view.
/// Despawned wholesale on each view transition.
#[derive(Component)]
pub struct ViewContent;

// ── Systems ───────────────────────────────────────────────────────────────────

pub fn on_nav_event(
    mut events: MessageReader<NavEvent>,
    mut state: ResMut<MenuState>,
    mut current_view: ResMut<CurrentView>,
) {
    for ev in events.read() {
        match ev {
            NavEvent::Up => {
                if state.selected == 0 {
                    state.selected = state.items.len() - 1;
                } else {
                    state.selected -= 1;
                }
            }
            NavEvent::Down => {
                state.selected = (state.selected + 1) % state.items.len();
            }
            NavEvent::Select => {
                match &state.items[state.selected].action {
                    MenuAction::GoTo(view) => current_view.0 = view.clone(),
                }
            }
            NavEvent::Back => {
                current_view.0 = View::MainMenu;
            }
        }
    }
}

/// Despawns all `ViewContent` entities and respawns content for the new view.
/// Fires on the first frame (resource init counts as changed) and on every
/// subsequent transition.
pub fn update_view(
    mut commands: Commands,
    current_view: Res<CurrentView>,
    state: Res<MenuState>,
    content: Query<Entity, With<ViewContent>>,
    asset_server: Res<AssetServer>,
) {
    if !current_view.is_changed() {
        return;
    }

    for entity in &content {
        commands.entity(entity).despawn();
    }

    let layer = RenderLayers::layer(CRT_LAYER);
    match &current_view.0 {
        View::MainMenu => spawn_main_menu(&mut commands, &state, &layer),
        View::Status   => spawn_status(&mut commands, &layer),
        View::Team     => spawn_placeholder(&mut commands, "TEAM", &layer),
        View::Map      => spawn_map(&mut commands, &asset_server, &layer),
        View::Account  => spawn_placeholder(&mut commands, "ACCOUNT", &layer),
    }
}

/// Refreshes menu row text + colour when the selection moves (MainMenu only).
pub fn update_text(
    state: Res<MenuState>,
    mut items: Query<(&MenuItem, &mut Text2d, &mut TextColor)>,
) {
    if !state.is_changed() {
        return;
    }
    for (item, mut text, mut color) in &mut items {
        let selected = item.index == state.selected;
        text.0 = format!("{} {}", if selected { ">" } else { " " }, item.label);
        color.0 = if selected {
            Color::WHITE
        } else {
            Color::srgba(0.55, 0.80, 0.55, 0.85)
        };
    }
}

// ── View spawners ─────────────────────────────────────────────────────────────

fn spawn_main_menu(commands: &mut Commands, state: &MenuState, layer: &RenderLayers) {
    // Title
    commands.spawn((
        Text2d::new("===  Scrap Gamblers  ==="),
        TextFont { font_size: 21.0, ..default() },
        TextColor(Color::srgb(0.90, 1.0, 0.90)),
        Transform::from_xyz(0.0, 128.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    // Divider
    commands.spawn((
        Text2d::new("----------------------------"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.50)),
        Transform::from_xyz(0.0, 104.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    // Menu items
    let item_spacing = 33.0_f32;
    let items_top    = 68.0_f32;
    for (i, item) in state.items.iter().enumerate() {
        let y        = items_top - i as f32 * item_spacing;
        let selected = i == state.selected;
        commands.spawn((
            Text2d::new(format!("{} {}", if selected { ">" } else { " " }, item.label)),
            TextFont { font_size: 19.0, ..default() },
            TextColor(if selected {
                Color::WHITE
            } else {
                Color::srgba(0.55, 0.80, 0.55, 0.85)
            }),
            Anchor::CENTER_LEFT,
            Transform::from_xyz(-SCREEN_W / 4.0, y, 0.0),
            layer.clone(),
            MenuItem { index: i, label: item.label },
            ViewContent,
        ));
    }
}

fn spawn_status(commands: &mut Commands, layer: &RenderLayers) {
    // Header
    commands.spawn((
        Text2d::new("[ STATUS ]"),
        TextFont { font_size: 17.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.70)),
        Transform::from_xyz(0.0, 128.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    // Divider
    commands.spawn((
        Text2d::new("----------------------------"),
        TextFont { font_size: 13.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.40)),
        Transform::from_xyz(0.0, 104.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    // Main status line
    commands.spawn((
        Text2d::new("SYSTEM ONLINE"),
        TextFont { font_size: 26.0, ..default() },
        TextColor(Color::srgb(0.40, 1.0, 0.50)),
        Transform::from_xyz(0.0, 30.0, 0.0),
        layer.clone(),
        ViewContent,
    ));
}

fn spawn_map(commands: &mut Commands, asset_server: &AssetServer, layer: &RenderLayers) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/map.png"),
            custom_size: Some(Vec2::new(400.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -10.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    // Header label above the map.
    commands.spawn((
        Text2d::new("[ MAP ]"),
        TextFont { font_size: 17.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.70)),
        Transform::from_xyz(0.0, 128.0, 1.0),
        layer.clone(),
        ViewContent,
    ));
}

fn spawn_placeholder(commands: &mut Commands, title: &'static str, layer: &RenderLayers) {
    let header = format!("[ {title} ]");
    commands.spawn((
        Text2d::new(header),
        TextFont { font_size: 17.0, ..default() },
        TextColor(Color::srgba(0.6, 0.9, 0.6, 0.70)),
        Transform::from_xyz(0.0, 128.0, 0.0),
        layer.clone(),
        ViewContent,
    ));

    commands.spawn((
        Text2d::new("-- COMING SOON --"),
        TextFont { font_size: 15.0, ..default() },
        TextColor(Color::srgba(0.5, 0.75, 0.5, 0.55)),
        Transform::from_xyz(0.0, 30.0, 0.0),
        layer.clone(),
        ViewContent,
    ));
}
