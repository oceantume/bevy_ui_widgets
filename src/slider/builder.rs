use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::{entity::*, *};
use bevy_utils::*;

use crate::utils::*;

use super::*;

/// Builds a slider widget
pub struct SliderWidgetBuilder<'a, 'w, 's> {
    root: WidgetBuilderEntity<'a, 'w, 's, Option<SliderBundle>>,
    track: WidgetBuilderEntity<'a, 'w, 's, Option<NodeBundle>>,
    thumb: WidgetBuilderEntity<'a, 'w, 's, Option<NodeBundle>>,
    tooltip_entity: Option<Entity>,
}

impl<'a, 'w, 's> SliderWidgetBuilder<'a, 'w, 's> {
    /// Creates a new slider builder
    pub fn new() -> Self {
        Self {
            root: WidgetBuilderEntity::new(Some(SliderBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            })),
            track: WidgetBuilderEntity::new(Some(NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Px(10.),
                        width: Val::Auto,
                    },
                    ..default()
                },
                color: Color::rgb(0.25, 0.25, 0.25).into(),
                border: Border {
                    width: 2.,
                    color: Color::rgb(0.15, 0.15, 0.15),
                },
                ..default()
            })),
            thumb: WidgetBuilderEntity::new(Some(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        height: Val::Px(16.),
                        width: Val::Px(16.),
                    },
                    ..default()
                },
                color: Color::rgb(0.25, 0.25, 0.25).into(),
                border: Border {
                    width: 2.,
                    color: Color::rgb(0.15, 0.15, 0.15),
                },
                ..default()
            })),
            tooltip_entity: None,
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

    /// Allows you to edit the root bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn root_bundle(&mut self, extend: impl FnOnce(SliderBundle) -> SliderBundle) -> &mut Self {
        self.root.bundle = Some(extend(std::mem::take(&mut self.root.bundle).unwrap()));
        self
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn track_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &Self {
        self.track.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows editing the track bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn track_bundle(&mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> &mut Self {
        self.track.bundle = Some(extend(std::mem::take(&mut self.track.bundle).unwrap()));
        self
    }

    /// Allows you to run commands on the root entity after it's spawned.
    pub fn thumb_commands(
        &mut self,
        run_commands: impl for<'b> Fn(&mut EntityCommands<'w, 's, 'b>) + 'a,
    ) -> &Self {
        self.thumb.commands_runners.push(Box::new(run_commands));
        self
    }

    /// Allows you to edit the thumb bundle before it is spawned.
    /// It is recommended to keep unmodified original values by using the struct extend syntax `..`.
    pub fn thumb_bundle(&mut self, extend: impl FnOnce(NodeBundle) -> NodeBundle) -> &mut Self {
        self.thumb.bundle = Some(extend(std::mem::take(&mut self.thumb.bundle).unwrap()));
        self
    }

    /// Consumes the builder, spawns the entity and returns the EntityCommands for the root node.
    /// Calling this will consume the builder. If you don't call this, entities will still be
    /// created and destroyed
    pub fn spawn(&mut self, commands: &'a mut Commands<'w, 's>) -> Entity {
        let root = commands
            .spawn_bundle(std::mem::take(&mut self.root.bundle).unwrap())
            .run_entity_commands(&self.root.commands_runners)
            .id();

        let track = commands
            .spawn_bundle(std::mem::take(&mut self.track.bundle).unwrap())
            .run_entity_commands(&self.track.commands_runners)
            .insert(SliderTrackNode)
            .insert(WidgetRoot(root))
            .id();

        let thumb = commands
            .spawn_bundle(std::mem::take(&mut self.thumb.bundle).unwrap())
            .run_entity_commands(&self.thumb.commands_runners)
            .insert(Interaction::None)
            .insert(SliderThumbNode)
            .insert(WidgetRoot(root))
            .id();

        commands.entity(root).push_children(&[track, thumb]);

        // if tooltip is created by user, it means we can't reliably edit content text.
        // content could be anything. we need to create it ourselves but offer abstraction?
        if let Some(tooltip_entity) = self.tooltip_entity {
            commands
                .entity(tooltip_entity)
                .insert(SliderTooltipNode)
                .insert(WidgetRoot(root));
        }

        root
    }
}
