use bevy_utils::HashMap;

use super::{ThemeProperty, ThemePropertyName};

pub struct ThemeManager {
    property_store: HashMap<String, Box<dyn ThemeProperty>>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            property_store: HashMap::new(),
        }
    }

    pub fn get_property<T>(&self, key: &str) -> Option<&T>
    where
        T: 'static + ThemeProperty + ThemePropertyName,
    {
        self.property_store
            .get(&format!("{}:{}", key, T::PROPERTY_NAME))
            .and_then(|result| {
                result.as_any().downcast_ref::<T>()
            })
    }

    pub fn set_property<T>(&mut self, key: &str, property: T) -> &mut Self
    where
        T: 'static + ThemeProperty + ThemePropertyName,
    {
        self.property_store.insert(
            format!("{}:{}", key, T::PROPERTY_NAME),
            Box::new(property),
        );
        self
    }
}
