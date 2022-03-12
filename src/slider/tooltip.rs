use bevy_ecs::prelude::*;
use bevy_text::*;
use bevy_transform::hierarchy::DespawnRecursiveExt;
use bevy_ui::{entity::*, *};

use super::*;

// Adds and removes tooltip nodes.
pub(crate) fn slider_tooltip(
    mut commands: Commands,
    added_slider_q: Query<(Entity, &SliderTooltip), Added<SliderTooltip>>,
    tooltip_q: Query<(Entity, &WidgetRoot)>,
    removed: RemovedComponents<SliderTooltip>,
) {
    for (root, slider_tooltip) in added_slider_q.iter() {
        let text = commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "0",
                    slider_tooltip.text_style.clone(),
                    Default::default(),
                ),
                ..Default::default()
            })
            .insert(SliderTooltipTextNode)
            .insert(WidgetRoot(root))
            .id();

        let tooltip = TooltipWidgetBuilder::new()
            .root_bundle(|bundle| TooltipBundle {
                position: TooltipPosition::FollowCursor,
                ..bundle
            })
            .with_content(text)
            .spawn(&mut commands);

        commands
            .entity(tooltip.root)
            .insert(UiColor(slider_tooltip.color))
            .insert(slider_tooltip.corner_radius)
            .insert(SliderTooltipNode)
            .insert(WidgetRoot(root));
    }

    for entity in removed.iter() {
        if let Some((entity, _)) = tooltip_q.iter().find(|(_, root)| root.0 == entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Updates the internal tooltip text style
///
/// TODO: Fix the change detection query. Otherwise this copies the style object every frame.
pub(crate) fn slider_tooltip_update(
    slider_q: Query<&SliderTooltip /*, Or<(Changed<SliderTooltip>, Changed<Children>)>*/>,
    mut tooltip_q: Query<(
        &WidgetRoot,
        &TooltipUiNodes,
        &mut UiColor,
        &mut CornerRadius,
    )>,
    mut tooltip_text_q: Query<&mut Text, With<TooltipTextUiNode>>,
) {
    for (root, nodes, mut color, mut corner_radius) in tooltip_q.iter_mut() {
        if let Ok(mut text) = tooltip_text_q.get_mut(nodes.text) {
            if let Ok(slider_tooltip) = slider_q.get(root.0) {
                text.sections[0].style = slider_tooltip.text_style.clone();
                color.0 = slider_tooltip.color;
                *corner_radius = slider_tooltip.corner_radius;
            }
        }
    }
}

pub(crate) fn slider_tooltip_visibility(
    thumb_q: Query<
        (&WidgetRoot, &Interaction, Option<&SliderThumbActive>),
        Or<(Changed<Interaction>, Changed<SliderThumbActive>)>,
    >,
    mut tooltip_q: Query<(&WidgetRoot, &mut Style), With<SliderTooltipNode>>,
) {
    for (root, interaction, _) in thumb_q.iter() {
        if let Some((_, mut style)) = tooltip_q.iter_mut().find(|(r, ..)| r.0 == root.0) {
            style.display = match interaction {
                Interaction::Clicked | Interaction::Hovered => Display::Flex,
                _ => Display::None,
            };
        }
    }
}

pub(crate) fn slider_tooltip_text_update(
    mut tooltip_text_q: Query<(&WidgetRoot, &mut Text), With<SliderTooltipTextNode>>,
    slider_q: Query<&Slider, (With<SliderTooltip>, Changed<Slider>)>,
) {
    for (root, mut text) in tooltip_text_q.iter_mut() {
        if let Ok(slider) = slider_q.get(root.0) {
            println!("Changing text to {}", slider.value);
            text.sections[0].value = slider.value.to_string();
        }
    }
}
