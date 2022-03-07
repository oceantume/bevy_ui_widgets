use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::{entity::*, *};

use super::*;

/// Builds a slider widget
pub struct SliderBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    style: Style,
    tooltip: Option<SliderTooltip>,
}

impl<'a, 'w, 's> SliderBuilder<'a, 'w, 's> {
    /// Creates a new slider builder
    pub fn new(commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            commands,
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            tooltip: None,
        }
    }

    pub fn with_tooltip(mut self, tooltip: SliderTooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn extend_style(mut self, extend: fn(Style) -> Style) -> Self {
        self.style = extend(self.style);
        self
    }

    /// Consumes the builder, spawns the entity and returns the EntityCommands for the root node.
    /// You can't use the builder after calling this.
    pub fn spawn(self) -> Entity {
        let root = self
            .commands
            .spawn_bundle(NodeBundle {
                style: self.style,
                ..Default::default()
            })
            .id();

        let track = self
            .commands
            .spawn_bundle(NodeBundle {
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
                corner_radius: CornerRadius::all(5.),
                ..Default::default()
            })
            .insert(SliderTrackNode)
            .insert(WidgetRoot(root))
            .id();

        let thumb = self
            .commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        // TODO: Implement this manually using real width.
                        left: Val::Px(0. - 8.),
                        ..Default::default()
                    },
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
                corner_radius: CornerRadius::all(4.),
                ..Default::default()
            })
            .insert(Interaction::None)
            .insert(SliderThumbNode)
            .insert(WidgetRoot(root))
            .id();

        self.commands.entity(root).push_children(&[track, thumb]);

        if let Some(tooltip) = self.tooltip {
            self.commands.entity(root).insert(tooltip);
        }

        root
    }
}
