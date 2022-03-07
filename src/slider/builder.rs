use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::{entity::*, *};

use super::*;

/// Builds a slider widget
pub struct SliderWidgetBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    tooltip: Option<SliderTooltip>,
    root_entity: Entity,
    root_bundle: SliderBundle,
    track_entity: Entity,
    track_bundle: NodeBundle,
    thumb_entity: Entity,
    thumb_bundle: NodeBundle,
}

impl<'a, 'w, 's> SliderWidgetBuilder<'a, 'w, 's> {
    /// Creates a new slider builder
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        let root_entity = commands.spawn().id();
        let track_entity = commands.spawn().id();
        let thumb_entity = commands.spawn().id();

        Self {
            commands,
            tooltip: None,
            root_entity,
            root_bundle: SliderBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                ..Default::default()
            },
            track_entity,
            track_bundle: NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Px(10.),
                        width: Val::Auto,
                    },
                    ..Default::default()
                },
                color: Color::rgb(0.25, 0.25, 0.25).into(),
                border: Border {
                    width: 2.,
                    color: Color::rgb(0.15, 0.15, 0.15),
                },
                ..Default::default()
            },
            thumb_entity,
            thumb_bundle: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        height: Val::Px(16.),
                        width: Val::Px(16.),
                    },
                    ..Default::default()
                },
                color: Color::rgb(0.25, 0.25, 0.25).into(),
                border: Border {
                    width: 2.,
                    color: Color::rgb(0.15, 0.15, 0.15),
                },
                ..Default::default()
            },
        }
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn root_commands(
        self,
        run_commands: impl for<'b> FnOnce(&mut EntityCommands<'w, 's, 'b>),
    ) -> Self {
        run_commands(&mut self.commands.entity(self.root_entity));
        self
    }

    /// Allows you to edit the root bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn root_bundle(mut self, extend: impl FnOnce(SliderBundle) -> SliderBundle) -> Self {
        self.root_bundle = extend(self.root_bundle);
        self
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn track_commands(
        self,
        run_commands: impl for<'b> FnOnce(&mut EntityCommands<'w, 's, 'b>),
    ) -> Self {
        run_commands(&mut self.commands.entity(self.track_entity));
        self
    }

    /// Allows editing the track bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn track_bundle(mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> Self {
        self.track_bundle = extend(self.track_bundle);
        self
    }

    /// Allows you to edit the thumb bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn thumb_bundle(mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> Self {
        self.thumb_bundle = extend(self.thumb_bundle);
        self
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn thumb_commands(
        self,
        run_commands: impl for<'b> FnOnce(&mut EntityCommands<'w, 's, 'b>),
    ) -> Self {
        run_commands(&mut self.commands.entity(self.thumb_entity));
        self
    }

    /// Consumes the builder, spawns the entity and returns the EntityCommands for the root node.
    /// You can't use the builder after calling this.
    pub fn spawn(self) -> Entity {
        let root = self.root_entity;

        self.commands.entity(root).insert_bundle(self.root_bundle);

        let track = self
            .commands
            .entity(self.track_entity)
            .insert_bundle(self.track_bundle)
            .insert(SliderTrackNode)
            .insert(WidgetRoot(root))
            .id();

        let thumb = self
            .commands
            .entity(self.thumb_entity)
            .insert_bundle(self.thumb_bundle)
            .insert(Interaction::None)
            .insert(SliderThumbNode)
            .insert(WidgetRoot(root))
            .id();

        self.commands.entity(root).push_children(&[track, thumb]);

        // TODO: replace this with an extensible TooltipBuilder.
        if let Some(tooltip) = self.tooltip {
            self.commands.entity(root).insert(tooltip);
        }

        root
    }
}
