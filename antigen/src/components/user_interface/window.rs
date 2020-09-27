use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Copy, Clone)]
pub struct Window;

impl ComponentDebugTrait for Window {
    fn get_name() -> String {
        "Window".into()
    }

    fn get_description() -> String {
        "Represents a unique window".into()
    }
}
