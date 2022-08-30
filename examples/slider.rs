use bevy::prelude::*;
use bevy_ui_widgets::{widgets::slider::*, AllWidgetsPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let slider = SliderWidgetBuilder::new()
        .root_bundle(|bundle| SliderBundle {
            slider: Slider {
                value: 50,
                min: 0,
                max: 100,
                step: 10,
            },
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                ..bundle.style
            },
            color: Color::NONE.into(),
            ..bundle
        })
        .spawn(&mut commands);

    commands.entity(slider.root).insert(SliderTooltip {
        text_style: TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 25.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
        color: Color::rgb(0.15, 0.15, 0.15),
    });
}
