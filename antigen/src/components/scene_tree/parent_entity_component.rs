use crate::ecs::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Debug, Default, Copy, Clone)]
pub struct ParentEntityComponent {
    parent_id: EntityID,
}

impl ParentEntityComponent {
    pub fn new(parent_id: EntityID) -> Self {
        ParentEntityComponent { parent_id }
    }

    pub fn get_parent_id(&self) -> EntityID {
        self.parent_id
    }

    pub fn set_parent_id(&mut self, parent_id: EntityID) -> &mut Self {
        self.parent_id = parent_id;
        self
    }
}

impl ComponentTrait for ParentEntityComponent {}

impl ComponentDebugTrait for ParentEntityComponent {
    fn get_name() -> String {
        "Parent".into()
    }

    fn get_description() -> String {
        "Parent Entity".into()
    }
}
