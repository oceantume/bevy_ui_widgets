use bevy::prelude::*;
use bevy_ui_widgets::{
    slider::{builder::SliderBuilder, *},
    AllWidgetsPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let slider = SliderBuilder::new(&mut commands)
        .extend_style(|style| Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Auto),
            ..style
        })
        .spawn();

    commands
        .entity(slider)
        .insert(Slider {
            value: 50,
            min: 0,
            max: 100,
            step: 10,
        })
        .insert(SliderTooltip {
            text_style: TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            color: Color::rgb(0.15, 0.15, 0.15),
            corner_radius: CornerRadius::all(2.0),
        })
        .insert(UiColor(Color::NONE));
}
