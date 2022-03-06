use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_text::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::{entity::*, *};

use super::*;

pub struct TooltipBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    color: Option<Color>,
    text: Option<Text>,
    position: Option<TooltipPosition>,
    align: Option<TooltipAlign>,
}

impl<'a, 'w, 's> TooltipBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new(commands: &'a mut Commands<'w, 's>) -> TooltipBuilder<'a, 'w, 's> {
        Self {
            commands,
            color: None,
            text: None,
            position: None,
            align: None,
        }
    }

    /// Sets the color for the tooltip's root node
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the text of the tooltip
    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    /// Sets the tooltip alignment
    pub fn with_align(mut self, align: TooltipAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Sets the tooltip position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Consumes the builder, spawns the entity and returns the EntityCommands for the root node.
    pub fn spawn(self) -> Entity {
        let root = self
            .commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    border: Rect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: self.color.unwrap_or_default().into(),
                ..Default::default()
            })
            .insert(Tooltip)
            .insert(self.position.unwrap_or_default())
            .insert(self.align.unwrap_or_default())
            .id();

        let text = self
            .commands
            .spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: self.text.unwrap_or_default(),
                ..Default::default()
            })
            .insert(TooltipTextUiNode(root))
            .id();

        self.commands
            .entity(root)
            .insert(TooltipUiNodes { root, text })
            .push_children(&[text]);
        root

        // TODO: Look into returning EntityCommands instead. I'm getting lifetime issues with it.
        // pub fn spawn<'b>(self) -> EntityCommands<'w, 's, 'b> {
        // self.commands.entity::<'b>(root)
    }
}
