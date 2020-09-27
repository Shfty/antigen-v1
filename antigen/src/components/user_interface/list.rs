use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Debug, Clone)]
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

impl Default for List {
    fn default() -> Self {
        List::new(None, None)
    }
}

impl ComponentTrait for List {}

impl ComponentDebugTrait for List {
    fn get_name() -> String {
        "List".into()
    }

    fn get_description() -> String {
        "String list UI control with an assemblage for customizing items".into()
    }
}
