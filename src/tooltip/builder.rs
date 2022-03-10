use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::*;
use bevy_utils::*;

use crate::utils::*;

use super::*;

pub struct TooltipWidgetBuilder<'a, 'w, 's> {
    root: WidgetBuilderEntity<'a, 'w, 's, Option<TooltipBundle>>,
    content_entity: Option<Entity>,
}

impl Default for TooltipWidgetBuilder<'_, '_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'w, 's> TooltipWidgetBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new() -> TooltipWidgetBuilder<'a, 'w, 's> {
        Self {
            root: WidgetBuilderEntity::new(Some(TooltipBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    border: Rect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })),
            content_entity: None,
        }
    }

    /// Allows to run commands on the root entity after it's spawned.
    pub fn root_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &mut Self {
        self.root.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows to edit the root bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn root_bundle(
        &mut self,
        extend: impl FnOnce(TooltipBundle) -> TooltipBundle,
    ) -> &mut Self {
        self.root.bundle = Some(extend(std::mem::take(&mut self.root.bundle).unwrap()));
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
            .spawn_bundle(std::mem::take(&mut self.root.bundle).unwrap())
            .run_entity_commands(&self.root.commands_runners)
            .id();

        if let Some(content) = self.content_entity {
            commands.entity(root_entity).add_child(content);
        }

        root_entity
    }
}
