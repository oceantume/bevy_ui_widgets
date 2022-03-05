#![allow(clippy::type_complexity)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod slider;
pub mod tooltip;

pub struct AllWidgetsPlugins;

impl PluginGroup for AllWidgetsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(tooltip::TooltipPlugin);
        group.add(slider::SliderPlugin);
    }
}
