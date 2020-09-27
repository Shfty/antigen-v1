use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::ColorRGBF,
};

impl ComponentTrait for char {}
impl ComponentTrait for String {}
impl ComponentTrait for Vec<String> {}
impl ComponentTrait for ColorRGBF {}

impl ComponentDebugTrait for char {
    fn get_name() -> String {
        "char".into()
    }

    fn get_description() -> String {
        "Primitive component containing a char".into()
    }
}

impl ComponentDebugTrait for String {
    fn get_name() -> String {
        "String".into()
    }

    fn get_description() -> String {
        "Primitive component containing a String".into()
    }
}

impl ComponentDebugTrait for Vec<String> {
    fn get_name() -> String {
        "Vec<String>".into()
    }

    fn get_description() -> String {
        "Primitive component containing a Vec<String>".into()
    }
}

impl ComponentDebugTrait for ColorRGBF {
    fn get_name() -> String {
        "ColorRGBF".into()
    }

    fn get_description() -> String {
        "Primitive component containing a ColorRGBF".into()
    }
}
