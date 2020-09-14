use crate::ecs::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Debug, Clone)]
pub struct ChildEntitiesComponent {
    child_ids: Vec<EntityID>,
}

impl ChildEntitiesComponent {
    pub fn new() -> Self {
        ChildEntitiesComponent {
            child_ids: Vec::new(),
        }
    }

    pub fn has_child_id(&self, child_id: &EntityID) -> bool {
        self.child_ids.contains(child_id)
    }

    pub fn add_child_id(&mut self, child_id: EntityID) -> &mut Self {
        self.child_ids.push(child_id);
        self
    }

    pub fn get_child_ids(&self) -> &Vec<EntityID> {
        self.child_ids.as_ref()
    }

    pub fn set_child_ids(&mut self, child_ids: Vec<EntityID>) -> &mut Self {
        self.child_ids = child_ids;
        self
    }
}

impl Default for ChildEntitiesComponent {
    fn default() -> Self {
        ChildEntitiesComponent::new()
    }
}

impl ComponentTrait for ChildEntitiesComponent {}

impl ComponentDebugTrait for ChildEntitiesComponent {
    fn get_name() -> String {
        "Child Entities".into()
    }

    fn get_description() -> String {
        "Holds child entity IDs".into()
    }
}
