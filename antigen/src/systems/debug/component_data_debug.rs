use crate::{
    components::DebugComponentDataList, components::DebugExclude, components::IntRange,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentID, entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
};
use crate::{
    components::EventQueue,
    entity_component_system::{SystemError, SystemTrait},
};

use super::{ComponentInspectorEvent, EntityInspectorEvent};

#[derive(Debug)]
pub struct ComponentDataDebug;

impl<CD> SystemTrait<CD> for ComponentDataDebug
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        // Populate entity components list
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<EventQueue<EntityInspectorEvent>>(entity_id)
                        && db.entity_has_component::<IntRange>(entity_id)
                });

        // Populate strings for debug component list entities
        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_index = db
                .get_entity_component::<IntRange>(entity_inspector_entity)?
                .get_index();

            if let Some(inspected_entity) = debug_entities.get(int_range_index as usize) {
                let mut components =
                    db.entity_component_directory
                        .get_components_by_predicate(|component_id| {
                            db.entity_component_directory
                                .entity_has_component_by_id(inspected_entity, component_id)
                        });

                components.sort_by_key(ComponentID::get_name);

                let component_inspector_entity = db
                    .entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_has_component::<EventQueue<ComponentInspectorEvent>>(entity_id)
                            && db.entity_has_component::<IntRange>(entity_id)
                    });

                if let Some(component_inspector_entity) = component_inspector_entity {
                    let int_range_index = db
                        .get_entity_component::<IntRange>(component_inspector_entity)?
                        .get_index();

                    if let Some(inspected_component) = components.get(int_range_index as usize) {
                        let component_data_id = db
                            .entity_component_directory
                            .get_entity_component_data_id(inspected_entity, inspected_component)?;

                         // FIXME: Reimplement debugging with new Store-based component setup
                        /*
                        let component_data_string = db
                            .component_storage
                            .get_component_data_string(&component_data_id)?;
                        */
                        let component_data_string =
                            format!("{}", component_data_id);

                        let entity_component_debug_entities = db
                            .entity_component_directory
                            .get_entities_by_predicate(|entity_id| {
                                db.entity_has_component::<DebugComponentDataList>(entity_id)
                                    && db.entity_has_component::<Vec<String>>(entity_id)
                            });

                        for entity_id in entity_component_debug_entities {
                            *db.get_entity_component_mut::<Vec<String>>(entity_id)? =
                                vec![component_data_string.clone()];
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
