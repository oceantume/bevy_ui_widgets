use bevy_math::Vec2;
use bevy_transform::components::GlobalTransform;
use bevy_ui::{CalculatedClip, Node};

pub fn get_uinode_clipped_rect(
    global_transform: &GlobalTransform,
    node: &Node,
    clip: Option<&CalculatedClip>,
) -> (Vec2, Vec2) {
    let position = global_transform.translation;
    let ui_position = position.truncate();
    let extents = node.size / 2.0;
    let mut min = ui_position - extents;
    let mut max = ui_position + extents;
    if let Some(clip) = clip {
        min = Vec2::max(min, clip.clip.min);
        max = Vec2::min(max, clip.clip.max);
    }
    (min, max)
}
