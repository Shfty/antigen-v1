use std::cell::{Ref, RefMut};

use store::StoreQuery;

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

        // Populate strings for debug component list entities
        if let Some((_, _, int_range)) = StoreQuery::<(
            EntityID,
            Ref<EventQueue<EntityInspectorEvent>>,
            Ref<IntRange>,
        )>::iter(db.component_store)
        .next()
        {
            if let Some(inspected_entity) = debug_entities.get(int_range.get_index() as usize) {
                let mut components =
                    db.entity_component_directory
                        .get_components_by_predicate(|component_id| {
                            db.entity_component_directory
                                .entity_has_component_by_id(inspected_entity, component_id)
                        });

                components.sort_by_key(ComponentID::get_name);

                if let Some((_, _, int_range)) = StoreQuery::<(
                    EntityID,
                    Ref<EventQueue<ComponentInspectorEvent>>,
                    Ref<IntRange>,
                )>::iter(db.component_store)
                .next()
                {
                    if let Some(inspected_component) =
                        components.get(int_range.get_index() as usize)
                    {
                        let component_data_id = db
                            .entity_component_directory
                            .get_entity_component_data_id(inspected_entity, inspected_component)?;

                        // FIXME: Reimplement debugging with new Store-based component setup
                        /*
                        let component_data_string = db
                            .component_storage
                            .get_component_data_string(&component_data_id)?;
                        */
                        let component_data_string = format!("{}", component_data_id);

                        if let Some((_, _, mut strings)) = StoreQuery::<(
                            EntityID,
                            Ref<DebugComponentDataList>,
                            RefMut<Vec<String>>,
                        )>::iter(
                            db.component_store
                        )
                        .next()
                        {
                            *strings = vec![component_data_string];
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
