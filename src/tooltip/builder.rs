use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::*;
use bevy_utils::*;
use smallvec::*;

use crate::utils::*;

use super::*;

pub struct TooltipWidgetBuilder<'a, 'w, 's> {
    root_bundle: Option<TooltipBundle>,
    content_entity: Option<Entity>,
    root_commands_runners: EntityCommandsRunnersVec<'a, 'w, 's>,
}

impl<'a, 'w, 's> TooltipWidgetBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new() -> TooltipWidgetBuilder<'a, 'w, 's> {
        Self {
            root_bundle: Some(TooltipBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    border: Rect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }),
            content_entity: None,
            root_commands_runners: default(),
        }
    }

    /// Allows to run commands on the root entity after it's spawned.
    pub fn root_commands(
        &mut self,
        run_commands: &'a dyn for<'b> Fn(&mut EntityCommands<'w, 's, 'b>),
    ) -> &mut Self {
        self.root_commands_runners.push(run_commands);
        self
    }

    /// Allows to edit the root bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn root_bundle(
        &mut self,
        extend: impl FnOnce(TooltipBundle) -> TooltipBundle,
    ) -> &mut Self {
        self.root_bundle = Some(extend(std::mem::take(&mut self.root_bundle).unwrap()));
        self
    }

    /// Sets the tooltip content.
    /// The entity should be a valid UI node and will be added to the tooltip's tree when spawn() is called.
    pub fn with_content(&mut self, entity: Entity) -> &mut Self {
        self.content_entity = Some(entity);
        self
    }

    /// Spawns the entity and returns the EntityCommands for the root node.
    /// Using the builder again after calling this will panic.
    pub fn spawn(&mut self, commands: &'a mut Commands<'w, 's>) -> Entity {
        let root_entity = commands
            .spawn_bundle(std::mem::take(&mut self.root_bundle).unwrap())
            .id();

        self.root_commands_runners
            .iter()
            .for_each(|run| run(&mut commands.entity(root_entity)));

        if let Some(content) = self.content_entity {
            commands.entity(root_entity).add_child(content);
        }

        root_entity
    }
}
