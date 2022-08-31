mod properties;
mod theme_manager;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::Text;
use bevy_ui::prelude::*;

pub use self::properties::*;
pub use self::theme_manager::*;

pub struct ThemingPlugin;

// TODO: properly handle Added and Changed for the key component.
// TODO: hierarchy traversal for inheritable properties (e.g. text color, font, size)

// Plugin that enables the systems for the theming module
impl Plugin for ThemingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThemeManager::new())
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(update_color)
                    .with_system(update_style)
                    .with_system(update_text_nodes),
            );
    }
}

#[derive(Component)]
pub struct ThemeKey(pub String);

impl From<&str> for ThemeKey {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

fn update_color(theme: Option<Res<ThemeManager>>, mut query: Query<(&ThemeKey, &mut UiColor)>) {
    if let Some(theme) = theme.filter(|style| style.is_added() || style.is_changed()) {
        for (key, mut value) in query.iter_mut() {
            if let Some(property) = theme
                .get_property::<ColorProperty>(&key.0)
                .filter(|color| color.0 != value.0)
            {
                value.0 = property.0;
            }
        }
    }
}

fn update_style(theme: Option<Res<ThemeManager>>, mut query: Query<(&ThemeKey, &mut Style)>) {
    if let Some(theme) = theme.filter(|style| style.is_added() || style.is_changed()) {
        for (key, mut style) in query.iter_mut() {
            macro_rules! change_style {
                ($a: ty, $b: expr) => {
                    if let Some(property) = theme.get_property::<$a>(&key.0) {
                        if $b != property.0 {
                            $b = property.0;
                        }
                    }
                };
            }

            change_style!(PaddingProperty, style.padding);
        }
    }
}

// TODO: support more properties for text nodes
// TODO: support having distinct properties for text sections
fn update_text_nodes(theme: Option<Res<ThemeManager>>, mut query: Query<(&ThemeKey, &mut Text)>) {
    if let Some(theme) = theme.filter(|style| style.is_added() || style.is_changed()) {
        for (key, mut text) in query.iter_mut() {
            if let Some(property) = theme.get_property::<TextColorProperty>(&key.0) {
                text.sections.iter_mut().for_each(|section| {
                    section.style.color = property.0;
                })
            }
        }
    }
}
