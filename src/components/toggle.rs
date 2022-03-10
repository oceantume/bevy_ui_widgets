use bevy_app::prelude::*;
use bevy_ecs::{prelude::*};
use bevy_ui::*;

pub struct ToggleComponentsPlugin;

// Plugin that enables the systems for the toggle components
impl Plugin for ToggleComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, toggle);
    }
}

/// The toggle state of a ui node. If the [`Interaction`] component is also present on the entity, this state will
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
        };
    }
}
