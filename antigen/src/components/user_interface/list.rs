use crate::entity_component_system::EntityID;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct List {
    string_list_entity: Option<EntityID>,
    selected_index: Option<usize>,
}

impl List {
    pub fn new(string_list_entity: Option<EntityID>) -> Self {
        List {
            string_list_entity,
            selected_index: None,
        }
    }

    pub fn get_string_list_entity(&self) -> Option<EntityID> {
        self.string_list_entity
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn set_selected_index(&mut self, selected_index: Option<usize>) {
        self.selected_index = selected_index
    }
}
