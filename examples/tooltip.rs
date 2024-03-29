use bevy::prelude::*;
use bevy_ui_widgets::{widgets::tooltip::*, AllWidgetsPlugins};

/// A simple tooltip that updates every frame.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .insert_resource(Counter(0))
        .add_startup_system(setup)
        .add_system(update_count)
        .run();
}

struct MyTooltipTextNode(Entity);
struct Counter(pub u32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let text = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            ..default()
        })
        .id();

    let _tooltip = TooltipWidgetBuilder::new()
        .root_bundle(|bundle| TooltipBundle {
            position: TooltipPosition::FollowCursor,
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..bundle
        })
        .with_content(text)
        .spawn(&mut commands);

    commands.insert_resource(MyTooltipTextNode(text));
}

fn update_count(
    mut query: Query<&mut Text>,
    text_node: Res<MyTooltipTextNode>,
    mut counter: ResMut<Counter>,
) {
    counter.0 += 1;

    if let Ok(mut text) = query.get_mut(text_node.0) {
        text.sections[0].value = counter.0.to_string();
    }
}
