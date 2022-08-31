use bevy::prelude::*;
use bevy_ui_widgets::{components::toggle::Toggle, theming::*, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .add_system(toggle_theme)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut theme: ResMut<ThemeManager>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // static theme variables
    theme
        .set_property("button", PaddingProperty(UiRect::all(Val::Px(10.0))))
        .set_property("root", JustifyContentProperty(JustifyContent::Center))
        .set_property("root", FlexDirectionProperty(FlexDirection::ColumnReverse));

    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::WHITE.into(),
            ..default()
        })
        .insert(ThemeKey("root".into()))
        .id();

    let text = commands
        .spawn_bundle(TextBundle::from_section(
            "Click here to toggle theme",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::RED,
            },
        ))
        .insert(UiColor(Color::rgb(0.5, 0.5, 0.0)))
        .insert(ThemeKey("text".into()))
        .id();

    let button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                //size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(ThemeKey("button".into()))
        .insert(Toggle::default())
        .add_child(text)
        .id();

    commands.entity(root).add_child(button);
}

/// Changes some theme properties when the button is toggled.
/// This basically shows how a light/dark mode could be achieved.
///
/// TODO: try to implement this via swapping pre-filled themes and using insert.
fn toggle_theme(mut query: Query<&Toggle, Changed<Toggle>>, mut theme: ResMut<ThemeManager>) {
    for toggle in query.iter_mut() {
        match toggle {
            Toggle::Off => {
                theme
                    .set_property("root", ColorProperty(Color::BLACK))
                    .set_property("text", TextColorProperty(Color::GREEN));
            }
            Toggle::On => {
                theme
                    .set_property("root", ColorProperty(Color::GRAY))
                    .set_property("text", TextColorProperty(Color::RED));
            }
        }
    }
}
