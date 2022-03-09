use bevy::prelude::*;
use bevy_ui_widgets::{components::{interaction::InteractionUiColor, toggle::{Toggle, ToggleUiColor}}, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(Toggle::default())
        .insert(ToggleUiColor {
            off: NORMAL_BUTTON,
            on: PRESSED_BUTTON,
        })
        .insert(InteractionUiColor {
            none: NORMAL_BUTTON,
            hovered: HOVERED_BUTTON,
            clicked: PRESSED_BUTTON,
        });
}
