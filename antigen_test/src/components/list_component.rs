use antigen::ecs::{Assemblage, ComponentMetadataTrait, ComponentTrait, EntityID};

#[derive(Debug, Clone)]
pub struct ListComponent {
    pub string_list_entity: Option<EntityID>,
    pub list_index_entity: Option<EntityID>,
    pub string_entity_assemblage: Option<Assemblage>,
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
}

impl Default for ListComponent {
    fn default() -> Self {
        ListComponent::new(None, None, None)
    }
}

impl ComponentTrait for ListComponent {}

impl ComponentMetadataTrait for ListComponent {
    fn get_name() -> &'static str {
        "List"
    }

    fn get_description() -> &'static str {
        "String list UI control with an assemblage for customizing items"
    }
}
