use bevy::prelude::*;
use bevy_ui_widgets::{
    components::toggle::Toggle,
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
        .add_system(minimize_button)
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
        .push_children(&[text, slider.root])
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

    let frame = FrameWidgetBuilder::new()
        .root_bundle(|bundle| NodeBundle {
            style: Style {
                position: Rect {
                    left: Val::Px(20.),
                    top: Val::Px(20.),
                    ..default()
                },
                // NOTE: min_size isn't working here for some reason.
                size: Size::new(Val::Px(250.0), Val::Undefined),
                ..bundle.style
            },
            color: Color::rgb(0.05, 0.05, 0.05).into(),
            ..bundle
        })
        .title_bar_bundle(|bundle| ButtonBundle {
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..bundle
        })
        .title_text_bundle(|bundle| TextBundle {
            text: Text::with_section(
                "Hello, My Frame!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                default(),
            ),
            ..bundle
        })
        .close_button_bundle(|bundle| ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..bundle.style
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..bundle
        })
        .close_button_commands(|commands| {
            let button = commands.id();
            let text = commands
                .commands()
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "-",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        default(),
                    ),
                    ..default()
                })
                .insert(RootEntity(button))
                .insert(MinimizeButtonText)
                .id();

            commands
                .insert(MinimizeButton)
                .insert(Toggle::default())
                .add_child(text);
        })
        .with_content(content)
        .spawn(&mut commands);

    commands.entity(content).insert(Content {
        frame_entity: frame.root,
    });
}

#[derive(Component)]
struct MinimizeButton;

#[derive(Component)]
struct MinimizeButtonText;

#[derive(Component)]
struct Content {
    frame_entity: Entity,
}

fn minimize_button(
    button_query: Query<(Entity, &RootEntity, &Toggle), (With<MinimizeButton>, Changed<Toggle>)>,
    mut button_text_query: Query<(&RootEntity, &mut Text), With<MinimizeButtonText>>,
    mut content_query: Query<(&Content, &mut Style)>,
) {
    for (button, root, toggle) in button_query.iter() {
        let content = content_query
            .iter_mut()
            .find(|(content, ..)| content.frame_entity == root.0);
        let text = button_text_query
            .iter_mut()
            .find(|(root, ..)| button == root.0);

        match toggle {
            Toggle::Off => {
                if let Some((_, mut style)) = content {
                    style.display = Display::Flex;
                }
                if let Some((_, mut text)) = text {
                    text.sections[0].value = "-".to_string();
                }
            }
            Toggle::On => {
                if let Some((_, mut style)) = content {
                    style.display = Display::None;
                }
                if let Some((_, mut text)) = text {
                    text.sections[0].value = "+".to_string();
                }
            }
        }
    }
}
