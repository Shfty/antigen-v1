use store::TypeKey;

use crate::entity_component_system::EntityID;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AntigenDebugEvent {
    SetInspectedEntity(EntityID),
    SetInspectedComponent(TypeKey),
    SetInspectedSystem(TypeKey),
}
