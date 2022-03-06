use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::view::Visibility;
use bevy_text::*;
use bevy_transform::{components::*, hierarchy::BuildChildren};
use bevy_ui::{entity::*, *};
use bevy_window::*;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tooltip_init_system)
            .add_system(position_update_system)
            .add_system(position_update_cursor_system)
            .add_system(update_text);
    }
}

/// Marker component present on all tooltips.
#[derive(Component, Default)]
pub struct Tooltip;

/// Describes the positioning of the tooltip.
///
/// TODO: Add uinode-relative positioning (using Entity + Node + ? )
#[derive(Component)]
pub enum TooltipPosition {
    /// Tooltip follows the cursor
    FollowCursor,
    /// Uses absolute positioning on the screen
    Absolute(Vec2),
    /// Raw rect. This is the same as using Manual and then setting style.rect
    Rect(Rect<Val>),
    /// Disables automatic positioning of the tooltip node.
    Manual,
}

/// Describes the alignment of the tooltip relative to its position.
/// This will be ignored when using a Rect position.
///
/// TODO: This should support combinations like BottomLeft, Left, TopLeft, Top, etc
#[derive(Component)]
pub enum TooltipAlign {
    Left,
    Right,
    Top,
    Bottom,
}

/// The tooltip's text.
///
/// When present, the text will automatically be copied to the tooltip's child text node.
#[derive(Component)]
pub struct TooltipText(pub Text);

#[derive(Component)]
pub struct TooltipUiNodes {
    /// Root node created via NodeBundle
    pub root: Entity,
    /// Text node created via TextBundle
    pub text: Entity,
}

/// Marker component used to identify the text node.
/// Also contains a reference to the tooltip's root entity.
#[derive(Component)]
pub struct TooltipTextUiNode(pub Entity);

fn tooltip_init_system(
    mut commands: Commands,
    tooltip_q: Query<(Entity, Option<&TooltipUiNodes>, Option<&TooltipText>), Added<Tooltip>>,
) {
    for (root, nodes, text) in tooltip_q.iter() {
        if nodes.is_none() {
            let text = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: text.map(|t| t.0.clone()).unwrap_or_default(),
                    ..Default::default()
                })
                .insert(TooltipTextUiNode(root))
                .id();

            commands.entity(root).add_child(text);
            commands.entity(root).insert(TooltipUiNodes { root, text });
        }
    }
}

fn position_update_system(
    mut tooltip_q: Query<
        (&TooltipPosition, &TooltipAlign, &mut Style),
        Or<(
            Changed<TooltipUiNodes>,
            Changed<TooltipPosition>,
            Changed<TooltipAlign>,
        )>,
    >,
) {
    for (position, _, mut style) in tooltip_q.iter_mut() {
        // TODO: handle TooltipAlign (we essentially change the top, left, right, bottom values accordingly)
        // TODO: handle collision with screen edges. we could probably use an invisible global node to get edges instead of Res<Window>.
        match position {
            TooltipPosition::Absolute(pos) => {
                style.position = Rect {
                    top: Val::Px(pos.y),
                    left: Val::Px(pos.x),
                    ..Default::default()
                };
            }
            TooltipPosition::Rect(rect) => {
                style.position = *rect;
            }
            _ => (),
        }
        //}
    }
}

fn position_update_cursor_system(
    mut tooltip_q: Query<(&TooltipPosition, &TooltipAlign, &mut Style), With<Tooltip>>,
    windows: Res<Windows>,
) {
    for (position, _, mut style) in tooltip_q.iter_mut() {
        if let TooltipPosition::FollowCursor = position {
            let window = windows.get_primary().unwrap();
            if let Some(pos) = window.cursor_position() {
                // TODO: We probably want to base this on the ui camera projection here.
                style.position = Rect {
                    top: Val::Px(window.height() - pos.y + 5.),
                    left: Val::Px(pos.x + 5.),
                    ..Default::default()
                }
            }
        }
    }
}

fn update_text(
    tooltips_q: Query<(&TooltipUiNodes, &TooltipText), Changed<TooltipText>>,
    mut text_node_q: Query<&mut Text, With<TooltipTextUiNode>>,
) {
    for (tooltip_nodes, tooltip_text) in tooltips_q.iter() {
        if let Ok(mut text) = text_node_q.get_mut(tooltip_nodes.text) {
            *text = tooltip_text.0.clone();
        }
    }
}

/// A tooltip is a text container with special positioning rules that displays on top of other UI elements.
/// The recommended way to use tooltips for now is to spawn them "standalone" and not as a child of another UI node.
#[derive(Bundle)]
pub struct TooltipBundle {
    pub tooltip: Tooltip,
    pub position: TooltipPosition,
    pub align: TooltipAlign,
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes the color of the node
    pub color: UiColor,
    /// Describes the image of the node
    pub image: UiImage,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Describes the radius of corners for the node
    pub corner_radius: CornerRadius,
    /// Describes the visual properties of the node's border
    pub border: Border,
}

impl Default for TooltipBundle {
    fn default() -> Self {
        Self {
            tooltip: Default::default(),
            position: TooltipPosition::FollowCursor,
            align: TooltipAlign::Right,
            node: Default::default(),
            style: Self::default_style(),
            color: Default::default(),
            image: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            corner_radius: Default::default(),
            border: Default::default(),
        }
    }
}

impl TooltipBundle {
    pub fn default_style() -> Style {
        Style {
            position_type: PositionType::Absolute,
            border: Rect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            ..Default::default()
        }
    }
}
