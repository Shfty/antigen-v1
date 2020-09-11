use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct WindowComponent;

impl ComponentTrait for WindowComponent {}

impl ComponentMetadataTrait for WindowComponent {
    fn get_name() -> &'static str {
        "Window"
    }

    fn get_description() -> &'static str {
        "Represents a unique window"
    }
}
