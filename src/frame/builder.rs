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
    close_button: WidgetBuilderEntity<'a, 'w, 's, Option<ButtonBundle>>,
    content_entity: Option<Entity>,
}

pub struct FrameWidgetEntities {
    pub root: Entity,
    pub title_bar: Entity,
    pub title_text: Entity,
    pub close_button: Entity,
    pub content: Option<Entity>,
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
                    align_items: AlignItems::Center,
                    min_size: Size {
                        height: Val::Px(25.0),
                        ..default()
                    },
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })),
            title_text: WidgetBuilderEntity::new(Some(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..default()
                },
                ..default()
            })),
            close_button: WidgetBuilderEntity::new(Some(ButtonBundle {
                style: Style {
                    //size: Size::new(Val::Px(35.0), Val::Px(35.0)),
                    aspect_ratio: Some(1.0),
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
    pub fn root_bundle(&mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> &mut Self {
        self.root.bundle = Some(extend(self.root.bundle.take().unwrap()));
        self
    }

    /// Allows to run commands on the title bar entity after it's spawned.
    pub fn title_bar_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &mut Self {
        self.title_bar.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows to edit the title bar bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn title_bar_bundle(&mut self, extend: impl FnOnce(ButtonBundle) -> ButtonBundle) -> &mut Self {
        self.title_bar.bundle = Some(extend(self.title_bar.bundle.take().unwrap()));
        self
    }

    /// Allows to run commands on the title text entity after it's spawned.
    pub fn title_text_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &mut Self {
        self.title_text.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows to edit the title text bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn title_text_bundle(&mut self, extend: impl FnOnce(TextBundle) -> TextBundle) -> &mut Self {
        self.title_text.bundle = Some(extend(self.title_text.bundle.take().unwrap()));
        self
    }

    /// Allows to run commands on the close button entity after it's spawned.
    pub fn close_button_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &mut Self {
        self.close_button.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows to edit the title close button bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn close_button_bundle(&mut self, extend: impl FnOnce(ButtonBundle) -> ButtonBundle) -> &mut Self {
        self.close_button.bundle = Some(extend(self.close_button.bundle.take().unwrap()));
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
    pub fn spawn(&mut self, commands: &'a mut Commands<'w, 's>) -> FrameWidgetEntities {
        let root = commands
            .spawn_bundle(self.root.bundle.take().unwrap())
            .insert(Frame)
            .run_entity_commands(&self.root.commands_runners)
            .id();

        let title_bar = commands
            .spawn_bundle(self.title_bar.bundle.take().unwrap())
            .insert(RootEntity(root))
            .insert(Grab)
            .insert(FrameGrabber)
            .run_entity_commands(&self.title_bar.commands_runners)
            .id();

        let title_text = commands
            .spawn_bundle(self.title_text.bundle.take().unwrap())
            .insert(RootEntity(root))
            .run_entity_commands(&self.title_text.commands_runners)
            .id();

        let close_button = commands
            .spawn_bundle(self.close_button.bundle.take().unwrap())
            .insert(RootEntity(root))
            .run_entity_commands(&self.close_button.commands_runners)
            .id();

        if let Some(content) = self.content_entity {
            commands.entity(root).add_child(content);
        }

        commands.entity(root).push_children(&[title_bar]);
        commands
            .entity(title_bar)
            .push_children(&[title_text, close_button]);

        FrameWidgetEntities {
            root,
            title_bar,
            title_text,
            close_button,
            content: self.content_entity,
        } 
    }
}
