use antigen::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ControlComponent;

impl ComponentTrait for ControlComponent {}

impl ComponentDebugTrait for ControlComponent {
    fn get_name() -> String {
        "Control".into()
    }

    fn get_description() -> String {
        "Tags an entity for rendering by UI systems".into()
    }
}
