use crate::entity_component_system::EntityID;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct List {
    string_list_entity: Option<EntityID>,
    list_index_entity: Option<EntityID>,
}

impl List {
    pub fn new(string_list_entity: Option<EntityID>, list_index_entity: Option<EntityID>) -> Self {
        List {
            string_list_entity,
            list_index_entity,
        }
    }

    pub fn get_string_list_entity(&self) -> Option<EntityID> {
        self.string_list_entity
    }

    pub fn get_list_index_entity(&self) -> Option<EntityID> {
        self.list_index_entity
    }
}
