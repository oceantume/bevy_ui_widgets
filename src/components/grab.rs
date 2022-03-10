use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_ui::*;
use bevy_window::prelude::*;

pub struct GrabComponentsPlugin;

/// Plugin that enables the systems for grab-related components
impl Plugin for GrabComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(grab)
            .add_system_to_stage(CoreStage::PreUpdate, grabbed_move);
    }
}

/// Added to a UI node that can be grabbed.
/// 
/// If [`Interaction`] is also present, the [`Grabbed`] component will automatically be
/// added and updated on that entity.
#[derive(Component, Copy, Clone, Debug)]
pub struct Grab;

/// Added to a UI node with [`Grab`] while it is actively grabbed.
#[derive(Component, Copy, Clone, Debug)]
pub struct Grabbed {
    /// Position of the window's cursor when grabbing started.
    pub cursor_position: Vec2,
    /// The current offset of the cursor relative to when grabbing started.
    pub cursor_offset: Vec2,
    /// The cursor offset during the previous update.
    pub previous_cursor_offset: Vec2,
}

fn grab(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, Option<&Grabbed>), With<Grab>>,
    windows: Res<Windows>,
) {
    for (entity, interaction, grabbed) in query.iter() {
        match interaction {
            Interaction::Clicked => {
                if grabbed.is_none() {
                    if let Some(cursor_position) = windows
                        .get_primary()
                        .and_then(|window| window.cursor_position())
                    {
                        commands.entity(entity).insert(Grabbed {
                            cursor_position,
                            cursor_offset: Vec2::new(0.0, 0.0),
                            previous_cursor_offset: Vec2::new(0.0, 0.0),
                        });
                    }
                }
            }
            _ => {
                if grabbed.is_some() {
                    commands.entity(entity).remove::<Grabbed>();
                }
            }
        };
    }
}

fn grabbed_move(mut query: Query<&mut Grabbed, With<Grab>>, windows: Res<Windows>) {
    for mut grabbed in query.iter_mut() {
        if let Some(cursor_position) = windows
            .get_primary()
            .and_then(|window| window.cursor_position())
        {
            let last_offset = grabbed.cursor_offset;
            let offset = cursor_position - grabbed.cursor_position;
            if grabbed.cursor_offset != offset {
                grabbed.cursor_offset = offset;
            }
            if grabbed.previous_cursor_offset != last_offset {
                grabbed.previous_cursor_offset = last_offset;
            }
        }
    }
}
