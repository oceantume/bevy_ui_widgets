use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::*;

use super::*;

pub struct TooltipWidgetBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    is_built: bool,
    root_entity: Entity,
    root_bundle: Option<TooltipBundle>,
    content_entity: Option<Entity>,
}

impl<'a, 'w, 's> TooltipWidgetBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new(commands: &'a mut Commands<'w, 's>) -> TooltipWidgetBuilder<'a, 'w, 's> {
        let root_entity = commands.spawn().id();

        Self {
            commands,
            is_built: false,
            root_entity,
            root_bundle: Some(TooltipBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    border: Rect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            }),
            content_entity: None,
        }
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn root_commands(
        &mut self,
        run_commands: impl for<'b> FnOnce(&mut EntityCommands<'w, 's, 'b>),
    ) -> &mut Self {
        run_commands(&mut self.commands.entity(self.root_entity));
        self
    }

    /// Allows you to edit the root bundle before it is spawned.
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

    /// Consumes the builder, spawns the entity and returns the EntityCommands for the root node.
    /// Using the builder again after calling this will panic.
    pub fn spawn(&mut self) -> Entity {
        assert!(
            !self.is_built,
            "TooltipWidgetBuilder must never be spawned more than once."
        );

        self.commands
            .entity(self.root_entity)
            .insert_bundle(std::mem::take(&mut self.root_bundle).unwrap());

        if let Some(content) = self.content_entity {
            self.commands.entity(self.root_entity).add_child(content);
        }

        self.is_built = true;
        self.root_entity
    }
}

impl<'a, 'w, 's> Drop for TooltipWidgetBuilder<'a, 'w, 's> {
    fn drop(&mut self) {
        assert!(self.is_built, "TooltipWidgetBuilder must always be spawned.");
    }
}
