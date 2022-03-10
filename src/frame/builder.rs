use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::prelude::*;
use bevy_ui::prelude::*;
use bevy_utils::*;

use crate::utils::*;

use super::*;

pub struct FrameWidgetBuilder<'a, 'w, 's> {
    root: WidgetBuilderEntity<'a, 'w, 's, Option<NodeBundle>>,
    title_bar: WidgetBuilderEntity<'a, 'w, 's, Option<ButtonBundle>>,
    title_text: WidgetBuilderEntity<'a, 'w, 's, Option<TextBundle>>,
    title_close_button: WidgetBuilderEntity<'a, 'w, 's, Option<ButtonBundle>>,
    content_entity: Option<Entity>,
}

impl Default for FrameWidgetBuilder<'_, '_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'w, 's> FrameWidgetBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new() -> Self {
        Self {
            root: WidgetBuilderEntity::new(Some(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::all(Val::Px(0.0)),
                    border: Rect::all(Val::Px(2.0)),
                    align_items: AlignItems::Stretch,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            })),
            title_bar: WidgetBuilderEntity::new(Some(ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    min_size: Size {
                        height: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            })),
            title_text: WidgetBuilderEntity::new(Some(default())),
            title_close_button: WidgetBuilderEntity::new(Some(ButtonBundle {
                style: Style {
                    //size: Size::new(Val::Px(35.0), Val::Px(35.0)),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                color: Color::GREEN.into(),
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
    pub fn root_bundle(&mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> &mut Self {
        self.root.bundle = Some(extend(std::mem::take(&mut self.root.bundle).unwrap()));
        self
    } 

    // TODO: Customization of all fields.

    pub fn title_bar_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &mut Self {
        self.title_bar.commands_runners.push(Box::new(run_commands));
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
        let root = commands
            .spawn_bundle(std::mem::take(&mut self.root.bundle).unwrap())
            .insert(Frame)
            .run_entity_commands(&self.root.commands_runners)
            .id();

        let title_bar = commands
            .spawn_bundle(std::mem::take(&mut self.title_bar.bundle).unwrap())
            .insert(RootEntity(root))
            .insert(Grab)
            .insert(FrameGrabber)
            .run_entity_commands(&self.title_bar.commands_runners)
            .id();

        let title_text = commands
            .spawn_bundle(std::mem::take(&mut self.title_text.bundle).unwrap())
            .insert(RootEntity(root))
            .run_entity_commands(&self.title_text.commands_runners)
            .id();

        let title_close_button = commands
            .spawn_bundle(std::mem::take(&mut self.title_close_button.bundle).unwrap())
            .insert(RootEntity(root))
            .run_entity_commands(&self.title_close_button.commands_runners)
            .id();

        if let Some(content) = self.content_entity {
            commands.entity(root).add_child(content);
        }

        commands.entity(root).push_children(&[title_bar]);
        commands
            .entity(title_bar)
            .push_children(&[title_text, title_close_button]);

        root
    }
}
