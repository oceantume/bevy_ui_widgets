use bevy::prelude::*;
use bevy_ui_widgets::{tooltip::*, AllWidgetsPlugins};

/// This is a more complex example of tooltip that show how to customize the internal nodes
/// of the tooltip.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .add_system(setup_text)
        .run();
}

struct MyTooltip(Entity);

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    let tooltip = commands
        .spawn_bundle(TooltipBundle {
            align: TooltipAlign::Left,
            position: TooltipPosition::FollowCursor,
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .id();

    commands.insert_resource(MyTooltip(tooltip))
}

fn setup_text(
    mut tooltip_q: Query<(&TooltipTextUiNode, &mut Text), Added<TooltipTextUiNode>>,
    my_tooltip: Res<MyTooltip>,
    asset_server: Res<AssetServer>,
) {
    if let Some((_, mut text)) = tooltip_q
        .iter_mut()
        .find(|(node, _)| node.0 == my_tooltip.0)
    {
        *text = Text::with_section(
            "Hello, tooltip!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        );
    }
}
