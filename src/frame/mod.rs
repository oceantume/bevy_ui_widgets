use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ui::prelude::*;
use bevy_utils::prelude::*;

mod builder;
pub use builder::*;

use crate::components::grab::{Grab, Grabbed};

pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_frame);
    }
}

/// Marker component for a frame's root.
#[derive(Component)]
pub struct Frame;

/// Marker component that indicates that a UI node can be used to grab the frame.
/// When present along with FrameRoot and Interaction, a system will automatically move
/// the frame around when this node is dragged.
///
/// NOTE: This could probably be changed for a more generic "UiNodeGrabber" with an Entity reference in the future.
///  However it would probably need to handle different styles and types of positioning, so TBD
#[derive(Component)]
pub struct FrameGrabber;

/// Describes a frame's root UI node.
#[derive(Component)]
pub struct RootEntity(pub Entity);

fn move_frame(
    query: Query<(&RootEntity, &Grabbed), (With<FrameGrabber>, Changed<Grabbed>)>,
    mut frame_query: Query<&mut Style, With<Frame>>,
) {
    for (root, grabbed) in query.iter() {
        if let Ok(mut style) = frame_query.get_mut(root.0) {
            let offset = grabbed.cursor_offset - grabbed.previous_cursor_offset;
            let x = match style.position.left {
                Val::Px(x) => x,
                _ => 0.0,
            };
            let y = match style.position.top {
                Val::Px(y) => y,
                _ => 0.0,
            };
            style.position = UiRect {
                left: Val::Px(x + offset.x),
                top: Val::Px(y - offset.y),
                ..default()
            }
        }
    }
}
