use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{color::Color, view::Visibility};
use bevy_text::*;
use bevy_transform::components::*;
use bevy_ui::*;

mod builder;
mod tooltip;

pub use builder::*;
use tooltip::*;

use crate::{components::grab::Grabbed, tooltip::*, utils::get_uinode_clipped_rect};

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(slider_test)
            .add_system(slider_thumb_update)
            .add_system(slider_thumb_move)
            .add_system_to_stage(CoreStage::PreUpdate, slider_tooltip)
            .add_system(slider_tooltip_update)
            .add_system(slider_tooltip_text_update)
            .add_system(slider_tooltip_visibility);
    }
}

// TODO: we may want to separate the "settings" from the "value"
// it could help especially with simplifying Changed<> queries.
#[derive(Component, Clone, Debug)]
pub struct Slider {
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: 0,
            min: 0,
            max: 100,
            step: 1,
        }
    }
}

/// When present, describes the visual appearance for the slider tooltip.
/// Remove this component to disable the slider's tooltip.
///
/// TODO: Improve this. It's not ideal to have to copy all those fields here.
#[derive(Component, Default, Clone)]
pub struct SliderTooltip {
    pub text_style: TextStyle,
    pub color: Color,
    pub corner_radius: CornerRadius,
}

/// Marker component for Slider's thumb
#[derive(Component)]
pub struct SliderThumbNode;

/// Marker component for Slider's track
#[derive(Component)]
pub struct SliderTrackNode;

/// Marker component for Slider's tooltip
#[derive(Component)]
pub struct SliderTooltipNode;

/// Marker component for Slider's tooltip text
#[derive(Component)]
pub struct SliderTooltipTextNode;

/// Marker component added to slider thumb while it's getting moved.
#[derive(Component)]
pub struct SliderThumbActive;

/// Added to an entity that's managed by a Widget.
/// It references the root entity for that Widget. The root is not necessarily the direct parent
/// of the node and in some cases it could be under another tree entirely (e.g. tooltips).
#[derive(Component)]
pub struct WidgetRoot(Entity);

fn slider_test(query: Query<&Node, Added<SliderTooltip>>) {
    for node in query.iter() {
        println!("{:?}", node);
    }
}

fn slider_thumb_update(
    mut thumb_q: Query<(&WidgetRoot, &Node, &mut Style), With<SliderThumbNode>>,
    track_q: Query<
        (
            &WidgetRoot,
            &Node,
            &GlobalTransform,
            Option<&CalculatedClip>,
        ),
        With<SliderTrackNode>,
    >,
    slider_q: Query<&Slider, Or<(Changed<Slider>, Changed<Node>, Changed<GlobalTransform>)>>,
) {
    for (root, thumb_node, mut thumb_style) in thumb_q.iter_mut() {
        if let Ok(slider) = slider_q.get(root.0) {
            if let Some((_, node, global_transform, clip)) = track_q
                .iter()
                .find(|(track_root, ..)| track_root.0 == root.0)
            {
                let (min_x, max_x) = {
                    let (min, max) = get_uinode_clipped_rect(global_transform, node, clip);
                    let min_x = min.x + (thumb_node.size.x / 2.0);
                    let max_x = max.x - (thumb_node.size.x / 2.0);
                    (min_x, max_x)
                };
                assert!(slider.step > 0 && slider.step <= slider.max - slider.min);
                let x = (slider.value as f32 * (max_x - min_x)) / (slider.max - slider.min) as f32;
                thumb_style.position = Rect {
                    left: Val::Px(x),
                    ..Default::default()
                };
            }
        }
    }
}

fn slider_thumb_move(
    thumb_q: Query<(&WidgetRoot, &Grabbed, &Node)>,
    track_q: Query<
        (
            &WidgetRoot,
            &Node,
            &GlobalTransform,
            Option<&CalculatedClip>,
        ),
        With<SliderTrackNode>,
    >,
    mut slider_q: Query<&mut Slider>,
) {
    for (root, grabbed, thumb_node) in thumb_q.iter() {
        if let Ok(mut slider) = slider_q.get_mut(root.0) {
            if let Some((_, node, global_transform, clip)) = track_q
                .iter()
                .find(|(track_root, ..)| track_root.0 == root.0)
            {
                let cursor_position = grabbed.cursor_position + grabbed.cursor_offset;

                let (min_x, max_x) = {
                    let (min, max) = get_uinode_clipped_rect(global_transform, node, clip);
                    let min_x = min.x + (thumb_node.size.x / 2.0);
                    let max_x = max.x - (thumb_node.size.x / 2.0);
                    (min_x, max_x)
                };

                let x = f32::clamp(cursor_position.x, min_x, max_x) - min_x;
                let mut value = ((x * (slider.max - slider.min) as f32) / (max_x - min_x)) as i32;

                assert!(slider.step > 0 && slider.step <= slider.max - slider.min);
                if slider.step > 1 {
                    value = ((value + slider.step / 2) / slider.step) * slider.step;
                }

                if slider.value != value {
                    slider.value = value;
                }
            }
        }
    }
}

/// A UI node that is a slider
#[derive(Bundle, Clone, Debug)]
/// Describes the value properties of the slider
pub struct SliderBundle {
    pub slider: Slider,
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

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            slider: Default::default(),
            node: Default::default(),
            style: Self::default_style(),
            color: Color::NONE.into(),
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

impl SliderBundle {
    pub fn default_style() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Stretch,
            ..Default::default()
        }
    }
}
