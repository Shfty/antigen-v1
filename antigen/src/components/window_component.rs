use crate::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct WindowComponent;

impl ComponentTrait for WindowComponent {}

impl ComponentDebugTrait for WindowComponent {
    fn get_name() -> String {
        "Window".into()
    }

    fn get_description() -> String {
        "Represents a unique window".into()
    }
}
