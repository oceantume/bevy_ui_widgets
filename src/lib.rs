#![allow(clippy::type_complexity)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod components;
pub mod utils;
pub mod theming;
pub mod widgets;

/// Plugin group used to add every plugins offered by bevy_ui_widget at once.
/// Use this if you don't want to mix and match which plugins you need.
pub struct AllWidgetsPlugins;

impl PluginGroup for AllWidgetsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(components::grab::GrabComponentsPlugin);
        group.add(components::toggle::ToggleComponentsPlugin);
        group.add(theming::ThemingPlugin);
        group.add(widgets::frame::FramePlugin);
        group.add(widgets::tooltip::TooltipPlugin);
        group.add(widgets::slider::SliderPlugin);
    }
}
