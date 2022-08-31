use std::any::Any;

use bevy_render::prelude::*;
use bevy_ui::prelude::*;

pub trait ThemePropertyName
where
    Self: ThemeProperty + Sized,
{
    /// The unique name for this property.
    /// Used for internal storage and serialization.
    const PROPERTY_NAME: &'static str;
}

pub trait ThemeProperty
where
    Self: Any + Send + Sync,
{
    fn as_any(&self) -> &dyn Any;
}

pub struct TextColorProperty(pub Color);

impl ThemePropertyName for TextColorProperty {
    const PROPERTY_NAME: &'static str = "color";
}

impl ThemeProperty for TextColorProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ColorProperty(pub Color);

impl ThemePropertyName for ColorProperty {
    const PROPERTY_NAME: &'static str = "background-color";
}

impl ThemeProperty for ColorProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct MarginProperty(pub UiRect<Val>);

impl ThemePropertyName for MarginProperty {
    const PROPERTY_NAME: &'static str = "margin";
}

impl ThemeProperty for MarginProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct PaddingProperty(pub UiRect<Val>);

impl ThemePropertyName for PaddingProperty {
    const PROPERTY_NAME: &'static str = "padding";
}

impl ThemeProperty for PaddingProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
