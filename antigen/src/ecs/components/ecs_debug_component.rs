use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Copy, Clone)]
pub enum DebugData {
    Entities,
    Components,
    ComponentData,
    EntityComponents
}

#[derive(Debug, Copy, Clone)]
pub struct ECSDebugComponent {
    pub debug_data: DebugData,
}

impl ECSDebugComponent {
    pub fn new(debug_data: DebugData) -> Self {
        ECSDebugComponent { debug_data }
    }
}

impl Default for ECSDebugComponent {
    fn default() -> Self {
        ECSDebugComponent {
            debug_data: DebugData::Entities,
        }
    }
}

impl ComponentTrait for ECSDebugComponent {}

impl ComponentMetadataTrait for ECSDebugComponent {
    fn get_name() -> &'static str {
        "ECS Debug"
    }

    fn get_description() -> &'static str {
        "Entrypoint for debug data"
    }
}
