use bevy_app::{PluginGroupBuilder, PluginGroup};

pub mod tooltip;

pub struct AllWidgetsPlugins;

impl PluginGroup for AllWidgetsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(tooltip::TooltipPlugin);
    }
}
