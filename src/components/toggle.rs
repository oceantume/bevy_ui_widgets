use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_ui::*;

pub struct ToggleComponentsPlugin;

impl Plugin for ToggleComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(toggle)
            .add_system(toggle_color)
            .add_system(toggle_image);
    }
}

/// The toggle state of a ui node. If the Interaction component is also present on the entity, this state will
/// be updated automatically from user clicks.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Toggle {
    Off,
    On,
}

impl Default for Toggle {
    fn default() -> Self {
        Self::Off
    }
}

fn toggle(mut query: Query<(&Interaction, &mut Toggle), Changed<Interaction>>) {
    for (interaction, mut toggle) in query.iter_mut() {
        if let Interaction::Clicked = interaction {
            *toggle = match *toggle {
                Toggle::On => Toggle::Off,
                Toggle::Off => Toggle::On,
            };
            println!("just set toggle to: {:?}", *toggle);
        };
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct ToggleUiColor {
    pub on: Color,
    pub off: Color,
}

fn toggle_color(
    mut query: Query<
        (&Toggle, &ToggleUiColor, &mut UiColor),
        Or<(Changed<Toggle>, Changed<ToggleUiColor>)>,
    >,
) {
    for (toggle, toggle_color, mut color) in query.iter_mut() {
        let new_value = match toggle {
            Toggle::On => toggle_color.on,
            Toggle::Off => toggle_color.off,
        };
        if color.0 != new_value {
            color.0 = new_value;
        }
    }
}

#[derive(Component)]
pub struct ToggleUiImage {
    pub on: Handle<Image>,
    pub off: Handle<Image>,
}

fn toggle_image(
    mut query: Query<
        (&Toggle, &ToggleUiImage, &mut Handle<Image>),
        Or<(Changed<Toggle>, Changed<ToggleUiImage>)>,
    >,
) {
    for (toggle, toggle_image, mut image) in query.iter_mut() {
        let new_value = match toggle {
            Toggle::On => &toggle_image.on,
            Toggle::Off => &toggle_image.off,
        };
        if *image != *new_value {
            *image = new_value.clone();
        }
    }
}
