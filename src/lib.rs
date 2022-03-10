#![allow(clippy::type_complexity)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod components;
pub mod frame;
pub mod slider;
pub mod tooltip;
pub mod utils;

/// Plugin group used to add every plugins offered by bevy_ui_widget at once.
/// Use this if you don't want to mix and match which plugins you need.
pub struct AllWidgetsPlugins;

impl PluginGroup for AllWidgetsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(frame::FramePlugin);
        group.add(tooltip::TooltipPlugin);
        group.add(slider::SliderPlugin);
        group.add(components::grab::GrabComponentsPlugin);
        group.add(components::toggle::ToggleComponentsPlugin);
    }
}
