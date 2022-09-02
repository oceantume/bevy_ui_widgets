mod properties;
mod theme_manager;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_text::prelude::*;
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

            change_style!(DisplayProperty, style.display);
            change_style!(PositionTypeProperty, style.position_type);
            change_style!(DirectionProperty, style.direction);
            change_style!(FlexDirectionProperty, style.flex_direction);
            change_style!(FlexWrapProperty, style.flex_wrap);
            change_style!(AlignItemsProperty, style.align_items);
            change_style!(AlignSelfProperty, style.align_self);
            change_style!(AlignContentProperty, style.align_content);
            change_style!(JustifyContentProperty, style.justify_content);
            change_style!(PositionProperty, style.position);
            change_style!(MarginProperty, style.margin);
            change_style!(PaddingProperty, style.padding);
            change_style!(BorderProperty, style.border);
            change_style!(FlexGrowProperty, style.flex_grow);
            change_style!(FlexShrinkProperty, style.flex_shrink);
            change_style!(FlexBasisProperty, style.flex_basis);
            change_style!(SizeProperty, style.size);
            change_style!(MinSizeProperty, style.min_size);
            change_style!(MaxSizeProperty, style.max_size);
            change_style!(AspectRatioProperty, style.aspect_ratio);
            change_style!(OverflowProperty, style.overflow);
        }
    }
}

// TODO: support more properties for text nodes
// TODO: support having distinct properties for text sections
fn update_text_nodes(
    theme: Option<Res<ThemeManager>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ThemeKey, &mut Text)>,
) {
    if let Some(theme) = theme.filter(|style| style.is_added() || style.is_changed()) {
        for (key, mut text) in query.iter_mut() {
            if let Some(property) = theme.get_property::<TextHorizontalAlignProperty>(&key.0) {
                if text.alignment.horizontal != property.0 {
                    text.alignment.horizontal = property.0;
                }
            }
            if let Some(property) = theme.get_property::<TextVerticalAlignProperty>(&key.0) {
                if text.alignment.vertical != property.0 {
                    text.alignment.vertical = property.0;
                }
            }
            if let Some(property) = theme.get_property::<TextColorProperty>(&key.0) {
                text.sections.iter_mut().for_each(|section| {
                    section.style.color = property.0;
                })
            }
            if let Some(property) = theme.get_property::<TextFontSizeProperty>(&key.0) {
                text.sections.iter_mut().for_each(|section| {
                    section.style.font_size = property.0;
                })
            }
            if let Some(property) = theme.get_property::<TextFontPathProperty>(&key.0) {
                text.sections.iter_mut().for_each(|section| {
                    let font_path = asset_server
                        .get_handle_path(section.style.font.clone());
                    if font_path.map_or(true, |path| path.path().as_os_str() != property.0.as_str()) {
                        section.style.font = asset_server.load(&property.0);
                    }
                })
            }
        }
    }
}
