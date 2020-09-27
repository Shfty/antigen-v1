use crate::{
    entity_component_system::ComponentID, entity_component_system::EntityID,
    entity_component_system::SystemID,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AntigenDebugEvent {
    SetInspectedEntity(EntityID),
    SetInspectedComponent(ComponentID),
    SetInspectedSystem(SystemID),
}
