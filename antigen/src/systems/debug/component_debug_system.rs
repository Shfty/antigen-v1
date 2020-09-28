use crate::entity_component_system::{SystemError, SystemTrait};
use crate::{
    components::DebugComponentList, components::DebugExclude, components::EntityInspector,
    components::IntRange, entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID, entity_component_system::SystemDebugTrait,
};

#[derive(Debug)]
pub struct ComponentDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for ComponentDebugSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_component_directory
                    .entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        // Populate entity components list
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EntityInspector>(entity_id)
                });

        // Populate strings for debug component list entities
        let debug_component_list_entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<DebugComponentList>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Vec<String>>(entity_id)
            });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_component =
                db.get_entity_component::<IntRange>(entity_inspector_entity)?;

            if let Some(inspected_entity) =
                debug_entities.get(int_range_component.get_index() as usize)
            {
                let mut components =
                    db.entity_component_directory
                        .get_components_by_predicate(|component_id| {
                            db.entity_component_directory
                                .entity_has_component_by_id(inspected_entity, component_id)
                        });

                components.sort_by(|lhs, rhs| {
                    let lhs_label = lhs.type_name;
                    let rhs_label = rhs.type_name;

                    lhs_label.cmp(&rhs_label)
                });

                let component_strings: Vec<&str> = components
                    .iter()
                    .map(|component_id| component_id.type_name)
                    .collect();

                for entity_id in debug_component_list_entities {
                    *db.get_entity_component_mut::<Vec<String>>(entity_id)? = component_strings
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect();
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for ComponentDebugSystem {
    fn get_name() -> &'static str {
        "Component Debug"
    }
}
