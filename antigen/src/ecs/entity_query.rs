use super::{component::ComponentID, ComponentTrait, EntityID, ECS};

pub struct EntityQueryBuilder<'a> {
    ecs: &'a mut ECS,
    component_ids: Vec<ComponentID>,
}

impl<'a> EntityQueryBuilder<'a> {
    pub fn new(ecs: &'a mut ECS) -> EntityQueryBuilder {
        EntityQueryBuilder {
            ecs,
            component_ids: Vec::new(),
        }
    }

    pub fn component<T: ComponentTrait + 'static>(&mut self) -> &'a mut EntityQueryBuilder {
        self.component_ids.push(ECS::get_component_id::<T>());
        self
    }

    pub fn finish(&mut self) -> Vec<EntityID> {
        self.ecs.get_entities_with_components(&self.component_ids)
    }
}
