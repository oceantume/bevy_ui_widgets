use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_ui::*;
use bevy_window::prelude::*;

use super::toggle::Toggle;

pub struct InteractionComponentsPlugin;

impl Plugin for InteractionComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(interaction_color)
            .add_system(interaction_image)
            .add_system(interaction_cursor);
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct InteractionUiColor {
    /// The node has been clicked
    pub clicked: Color,
    /// The node has been hovered over
    pub hovered: Color,
    /// Nothing has happened
    pub none: Color,
}

fn interaction_color(
    mut query: Query<
        (&Interaction, &InteractionUiColor, &mut UiColor),
        (
            Without<Toggle>,
            Or<(Changed<Interaction>, Changed<InteractionUiColor>)>,
        ),
    >,
) {
    for (interaction, interaction_color, mut color) in query.iter_mut() {
        let new_value = match interaction {
            Interaction::Clicked => interaction_color.clicked,
            Interaction::Hovered => interaction_color.hovered,
            Interaction::None => interaction_color.none,
        };
        if color.0 != new_value {
            color.0 = new_value;
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct InteractionUiImage {
    /// The node has been clicked
    pub clicked: Handle<Image>,
    /// The node has been hovered over
    pub hovered: Handle<Image>,
    /// Nothing has happened
    pub none: Handle<Image>,
}

fn interaction_image(
    mut query: Query<
        (&Interaction, &InteractionUiImage, &mut UiImage),
        (
            Without<Toggle>,
            Or<(Changed<Interaction>, Changed<InteractionUiImage>)>,
        ),
    >,
) {
    for (interaction, interaction_image, mut image) in query.iter_mut() {
        let new_value = match interaction {
            Interaction::Clicked => &interaction_image.clicked,
            Interaction::Hovered => &interaction_image.hovered,
            Interaction::None => &interaction_image.none,
        };
        if image.0 != *new_value {
            image.0 = new_value.clone();
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct InteractionCursorIcon {
    pub clicked: CursorIcon,
    pub hovered: CursorIcon,
    pub none: CursorIcon,
}

impl Default for InteractionCursorIcon {
    fn default() -> Self {
        Self {
            clicked: CursorIcon::Default,
            hovered: CursorIcon::Default,
            none: CursorIcon::Default,
        }
    }
}

fn interaction_cursor(
    query: Query<
        (&Interaction, &InteractionCursorIcon),
        (Or<(Changed<Interaction>, Changed<InteractionCursorIcon>)>),
    >,
    mut windows: ResMut<Windows>,
) {
    for (interaction, interaction_icon) in query.iter() {
        if let Some(window) = windows.get_primary_mut() {
            let icon = match interaction {
                Interaction::Clicked => interaction_icon.clicked,
                Interaction::Hovered => interaction_icon.hovered,
                Interaction::None => interaction_icon.none,
            };
            window.set_cursor_icon(icon);
        }
    }
}
