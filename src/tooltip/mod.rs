use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_text::prelude::*;
use bevy_transform::prelude::*;
use bevy_ui::{prelude::*, FocusPolicy};
use bevy_utils::prelude::*;
use bevy_window::prelude::*;

mod builder;
pub use builder::*;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, position_update_system)
            .add_system_to_stage(CoreStage::PreUpdate, position_update_cursor_system)
            .add_system_to_stage(CoreStage::PreUpdate, position_update_node_system)
            .add_system(update_text);
    }
}

/// Marker component present on all tooltips.
#[derive(Component, Default, Debug)]
pub struct Tooltip;

/// Describes the positioning of the tooltip.
///
/// TODO: Add uinode-relative positioning (using Entity + Node + ? )
#[derive(Component, Debug)]
pub enum TooltipPosition {
    /// Tooltip follows the cursor
    FollowCursor,
    /// Tooltip is positioned relative to a node.
    Node(Entity),
    /// Uses absolute positioning on the screen
    Absolute(Vec2),
    /// Raw rect. This is the same as using Manual and then setting style.rect
    Rect(UiRect<Val>),
    /// Disables automatic positioning of the tooltip node.
    Manual,
}

impl Default for TooltipPosition {
    fn default() -> Self {
        Self::Manual
    }
}

/// Describes the alignment of the tooltip relative to its position.
/// This will be ignored when using a Rect position.
#[derive(Component, Debug)]
pub enum TooltipAlign {
    Bottom,
    //BottomLeft,
    //BottomRight,
    Left,
    Right,
    Top,
    //TopLeft,
    //TopRight,
}

impl Default for TooltipAlign {
    fn default() -> Self {
        Self::Top
    }
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

fn position_update_system(
    mut tooltip_q: Query<
        (&TooltipPosition, &TooltipAlign, &mut Style),
        Or<(
            Changed<TooltipUiNodes>,
            Changed<TooltipPosition>,
            Changed<TooltipAlign>,
        )>,
    >,
    //node_query: Query<(&Node, &GlobalTransform)>,
) {
    for (position, _, mut style) in tooltip_q.iter_mut() {
        // TODO: handle TooltipAlign (we essentially change the top, left, right, bottom values accordingly)
        // TODO: handle collision with screen edges. we could probably use an invisible global node to get edges instead of Res<Window>.
        match position {
            TooltipPosition::Absolute(pos) => {
                style.position = UiRect {
                    top: Val::Px(pos.y),
                    left: Val::Px(pos.x),
                    ..default()
                };
            }
            TooltipPosition::Rect(rect) => {
                style.position = *rect;
            }
            _ => (),
        }
    }
}

fn position_update_cursor_system(
    mut tooltip_q: Query<(&TooltipPosition, &TooltipAlign, &Node, &mut Style), With<Tooltip>>,
    windows: Res<Windows>,
) {
    for (position, align, node, mut style) in tooltip_q.iter_mut() {
        if let TooltipPosition::FollowCursor = position {
            let window = windows.get_primary().unwrap();
            if let Some(pos) = window.cursor_position() {
                style.position = calculate_tooltip_rect(align, &node.size, &pos, 5.0);
            }
        }
    }
}

fn position_update_node_system(
    mut tooltip_q: Query<
        (
            &TooltipPosition,
            &TooltipAlign,
            &GlobalTransform,
            &Node,
            &mut Style,
        ),
        With<Tooltip>,
    >,
    node_query: Query<(&GlobalTransform, &Node)>,
) {
    for (position, align, _transform, node, mut style) in tooltip_q.iter_mut() {
        if let TooltipPosition::Node(position_entity) = position {
            if let Ok((other_transform, other_node)) = node_query.get(*position_entity) {
                if node.size == Vec2::splat(0.0) {
                    // NOTE: node size is 0.0 for one frame when we update display, because it was not yet calculated.
                    //  we will have to find a solution at some point, but for now this is a good-enough hack around it.
                    return;
                }

                let point = calculate_node_point(
                    align,
                    &other_node.size,
                    &other_transform.translation().truncate(),
                );
                let rect = calculate_tooltip_rect(align, &node.size, &point, 2.0);
                if style.position != rect {
                    style.position = rect;
                }
            }
        }
    }
}

fn calculate_node_point(
    align: &TooltipAlign,
    size: &Vec2,
    position: &Vec2,
) -> Vec2 {
    match align {
        TooltipAlign::Bottom => Vec2::new(position.x, position.y - (size.y / 2.0)),
        TooltipAlign::Left => Vec2::new(position.x - (size.x / 2.0), position.y),
        TooltipAlign::Right => Vec2::new(position.x + (size.x / 2.0), position.y),
        TooltipAlign::Top => Vec2::new(position.x, position.y + (size.y / 2.0)),
    }
}

fn calculate_tooltip_rect(
    align: &TooltipAlign,
    tooltip_size: &Vec2,
    point: &Vec2,
    offset: f32,
) -> UiRect<Val> {
    match align {
        TooltipAlign::Bottom => UiRect {
            bottom: Val::Px(point.y - tooltip_size.y - offset),
            left: Val::Px(point.x - (tooltip_size.x / 2.0)),
            ..default()
        },
        TooltipAlign::Left => UiRect {
            left: Val::Px(point.x - tooltip_size.x - offset),
            bottom: Val::Px(point.y - (tooltip_size.y / 2.0)),
            ..default()
        },
        TooltipAlign::Right => UiRect {
            left: Val::Px(point.x + offset),
            bottom: Val::Px(point.y - (tooltip_size.y / 2.0)),
            ..default()
        },
        TooltipAlign::Top => UiRect {
            bottom: Val::Px(point.y + offset),
            left: Val::Px(point.x - (tooltip_size.x / 2.0)),
            ..default()
        },
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
#[derive(Bundle, Debug)]
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
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for TooltipBundle {
    fn default() -> Self {
        Self {
            tooltip: default(),
            position: TooltipPosition::FollowCursor,
            align: TooltipAlign::Right,
            node: default(),
            style: Self::default_style(),
            color: default(),
            image: default(),
            focus_policy: default(),
            transform: default(),
            global_transform: default(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

impl TooltipBundle {
    pub fn default_style() -> Style {
        Style {
            position_type: PositionType::Absolute,
            border: UiRect::all(Val::Px(2.0)),
            align_items: AlignItems::Center,
            ..default()
        }
    }
}
