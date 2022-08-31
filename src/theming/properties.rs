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

macro_rules! define_property {
    ($type: ident, $inner_type: ty, $name_str: expr) => {
        pub struct $type(pub $inner_type);

        impl ThemePropertyName for $type {
            const PROPERTY_NAME: &'static str = $name_str;
        }

        impl ThemeProperty for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

define_property!(TextColorProperty, Color, "color");
define_property!(ColorProperty, Color, "background-color");
define_property!(PaddingProperty, UiRect<Val>, "padding");
