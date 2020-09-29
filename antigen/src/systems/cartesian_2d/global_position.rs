use crate::{
    components::{GlobalPositionData, ParentEntity, Position},
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::EntityID,
};
use crate::{
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

#[derive(Debug)]
pub struct GlobalPosition;

impl Default for GlobalPosition {
    fn default() -> Self {
        GlobalPosition
    }
}

impl GlobalPosition {
    pub fn new() -> Self {
        GlobalPosition::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for GlobalPosition
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<Position>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<ParentEntity>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<GlobalPositionData>(entity_id)
            });

        for entity_id in entities {
            let parent_entity: EntityID =
                (*db.get_entity_component::<ParentEntity>(entity_id)?).into();

            let position = *db.get_entity_component::<Position>(entity_id)?;
            let mut global_position: Vector2I = position.into();
            let mut candidate_id = parent_entity;

            loop {
                let parent_position = *db.get_entity_component::<Position>(candidate_id)?;
                global_position += parent_position.into();

                if db
                    .get_entity_component::<GlobalPositionData>(candidate_id)
                    .is_err()
                {
                    break;
                }

                match db.get_entity_component::<ParentEntity>(candidate_id) {
                    Ok(parent_entity_component) => candidate_id = (*parent_entity_component).into(),
                    Err(_) => break,
                }
            }

            *db.get_entity_component_mut::<GlobalPositionData>(entity_id)? =
                global_position.into();
        }

        Ok(())
    }
}

