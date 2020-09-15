use antigen::entity_component_system::{Assemblage, ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Debug, Clone)]
pub struct ListComponent {
    string_list_entity: Option<EntityID>,
    list_index_entity: Option<EntityID>,
    string_entity_assemblage: Option<Assemblage>,
}

impl ListComponent {
    pub fn new(
        string_list_entity: Option<EntityID>,
        list_index_entity: Option<EntityID>,
        string_entity_assemblage: Option<Assemblage>,
    ) -> Self {
        ListComponent {
            string_list_entity,
            list_index_entity,
            string_entity_assemblage,
        }
    }

    pub fn get_string_list_entity(&self) -> Option<EntityID> {
        self.string_list_entity
    }

    pub fn get_list_index_entity(&self) -> Option<EntityID> {
        self.list_index_entity
    }

    pub fn get_string_entity_assemblage(&self) -> Option<&Assemblage> {
        self.string_entity_assemblage.as_ref()
    }
}

impl Default for ListComponent {
    fn default() -> Self {
        ListComponent::new(None, None, None)
    }
}

impl ComponentTrait for ListComponent {}

impl ComponentDebugTrait for ListComponent {
    fn get_name() -> String {
        "List".into()
    }

    fn get_description() -> String {
        "String list UI control with an assemblage for customizing items".into()
    }
}
