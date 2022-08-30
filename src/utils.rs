use bevy_ecs::system::EntityCommands;
use bevy_math::Vec2;
use bevy_transform::components::GlobalTransform;
use bevy_ui::{CalculatedClip, Node};
use bevy_utils::default;
use smallvec::SmallVec;

pub fn get_uinode_clipped_rect(
    global_transform: &GlobalTransform,
    node: &Node,
    clip: Option<&CalculatedClip>,
) -> (Vec2, Vec2) {
    let position = global_transform.translation();
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

/// A shortcut type for lists of entity commands runners.
///
/// This uses a SmallVec to optimize space since two entries should be enough for 90% of use-cases.
pub type EntityCommandsRunnersVec<'a, 'w, 's> =
    SmallVec<[Box<dyn for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a>; 2]>;


pub struct WidgetBuilderEntity<'a, 'w, 's, TBundle> {
    pub commands_runners: EntityCommandsRunnersVec<'a, 'w, 's>,
    pub bundle: TBundle,
}

impl<'a, 'w, 's, TBundle> WidgetBuilderEntity<'a, 'w, 's, TBundle> {
    pub fn new(bundle: TBundle) -> Self {
        Self {
            bundle,
            commands_runners: default(),
        }
    }
}

pub trait RunEntityCommands<'w, 's, 'a> {
    fn run_entity_commands(
        &mut self,
        runners: &[Box<dyn for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a>],
    ) -> &mut Self;
}

impl<'w, 's, 'a> RunEntityCommands<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {
    fn run_entity_commands(
        &mut self,
        runners: &[Box<dyn for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a>],
    ) -> &mut Self {
        runners.iter().for_each(|run| run(self));
        self
    }
}
