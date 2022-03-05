use bevy::prelude::*;
use bevy_ui_widgets::{slider::*, AllWidgetsPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SliderBundle {
        slider: Slider {
            value: 50,
            min: 0,
            max: 100,
            step: 10,
            ..Default::default()
        },
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Auto),
            ..SliderBundle::default_style()
        },
        ..default()
    });
}
