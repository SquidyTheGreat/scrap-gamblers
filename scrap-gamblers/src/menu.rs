use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuState {
    pub items: Vec<&'static str>,
    pub selected: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            items: vec!["STATUS", "SPECIAL", "INVENTORY", "DATA", "MAP", "RADIO"],
            selected: 0,
        }
    }
}

/// Navigation messages fired by buttons and keyboard.
#[derive(Message, Clone, Copy)]
pub enum NavEvent {
    Up,
    Down,
    Select,
}

/// Marks the 2-D text entities that represent menu rows (rendered into the
/// CRT render-target camera).
#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
    pub label: &'static str,
}

pub fn on_nav_event(
    mut events: MessageReader<NavEvent>,
    mut state: ResMut<MenuState>,
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
                // Future: dispatch action for state.items[state.selected]
            }
        }
    }
}

/// Refreshes text content + colour whenever the selection changes.
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
