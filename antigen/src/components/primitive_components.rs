use crate::{entity_component_system::ComponentDebugTrait, primitive_types::ColorRGBF};

impl ComponentDebugTrait for char {}
impl ComponentDebugTrait for String {}
impl ComponentDebugTrait for Vec<String> {}
impl ComponentDebugTrait for ColorRGBF {}
