use bevy::prelude::*;
use bevy_ui_widgets::{
    frame::*,
    slider::{SliderBundle, SliderWidgetBuilder},
    AllWidgetsPlugins,
};

/// An advanced UI example using many different widgets and components.
/// TODO: Move this to somewhere else and make a super simple frame.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    let text = commands
        .spawn_bundle(TextBundle {
            style: Style { ..default() },
            text: Text::with_section(
                "Slider:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..default()
        })
        .id();

    let slider = SliderWidgetBuilder::new()
        .root_bundle(|bundle| SliderBundle {
            style: Style {
                size: Size::new(Val::Px(150.), Val::Auto),
                ..bundle.style
            },
            ..bundle
        })
        .spawn(&mut commands);

    let section = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .push_children(&[text, slider])
        .id();

    let content = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Stretch,
                //justify_content: JustifyContent::Center,
                margin: Rect::all(Val::Px(5.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .push_children(&[section])
        .id();

    FrameWidgetBuilder::new()
        .root_bundle(|bundle| NodeBundle {
            style: Style {
                position: Rect {
                    left: Val::Px(20.),
                    top: Val::Px(20.),
                    ..default()
                },
                size: Size::new(Val::Px(300.0), Val::Px(200.0)),
                ..bundle.style
            },
            color: Color::rgb(0.05, 0.05, 0.05).into(),
            ..bundle
        })
        .with_content(content)
        .spawn(&mut commands);
}
