use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ControlComponent;

impl ComponentTrait for ControlComponent {}

impl ComponentMetadataTrait for ControlComponent {
    fn get_name() -> &'static str {
        "Control"
    }

    fn get_description() -> &'static str {
        "Tags an entity for rendering by UI systems"
    }
}
