use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::hierarchy::BuildChildren;
use bevy_ui::{entity::*, *};

use super::*;

pub struct TooltipBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    color: Option<Color>,
    content: Option<Entity>,
    position: Option<TooltipPosition>,
    align: Option<TooltipAlign>,
}

impl<'a, 'w, 's> TooltipBuilder<'a, 'w, 's> {
    /// Creates a new tooltip builder
    pub fn new(commands: &'a mut Commands<'w, 's>) -> TooltipBuilder<'a, 'w, 's> {
        Self {
            commands,
            color: None,
            content: None,
            position: None,
            align: None,
        }
    }

    /// Sets the color for the tooltip's root node
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
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

    /// Sets the tooltip content.
    /// The entity should be a valid UI node and will be added to the tooltip's tree when spawn() is called.
    pub fn with_content(mut self, entity: Entity) -> Self {
        self.content = Some(entity);
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

        if let Some(content) = self.content {
            self.commands.entity(root).add_child(content);
        }

        root

        // TODO: Look into returning EntityCommands instead. I'm getting lifetime issues with it.
        // pub fn spawn<'b>(self) -> EntityCommands<'w, 's, 'b> {
        // self.commands.entity::<'b>(root)
    }
}
